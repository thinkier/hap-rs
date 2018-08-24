# HAP (HomeKit Accessory Protocol)

<!--- [![Build Status](https://travis-ci.org/korhalio/hap-rs.svg?branch=master)](https://travis-ci.org/korhalio/hap-rs) --->
<!--- [![Latest Version](https://img.shields.io/crates/v/hap.svg)](https://crates.io/crates/hap) --->
<!--- [![docs](https://docs.rs/hap/badge.svg)](https://docs.rs/hap) --->
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://www.gnu.org/licenses/gpl-3.0)

Rust implementation of the Apple HomeKit Accessory Protocol (HAP) based on [Tokio](https://github.com/tokio-rs/tokio) and [Hyper](https://github.com/hyperium/hyper).

This crate supports all HomeKit Services and Characteristics currently implemented by Apple and provides the ability to create custom Characteristics, Services and Accessories.

The HomeKit Accessory Protocol supports transports over IP and Bluetooth LE. Currently only the transport over IP is implemented in this crate. Accessories are exposed by the implemented HAP Accessory HTTP server and announced via built-in mDNS.


## HomeKit Data Model

The HAP defines HomeKit enabled devices as virtual `Accessories` that are composed of `Services` that are composed of `Characteristics`.

Characteristics hold values of various data types as well as optional metadata like max/min values or units. Services group Characteristics and represent features of the Accessory. Every Accessory consist of at least one `Accessory Information Service` and any number of additional Services. For example a custom ceiling fan Accessory may consist of an `Accessory Information Service`, a `Fan Service` and a `Lightbulb Service`.

```
Ceiling Fan Accessory
|
|-- Accessory Information Service
|   |-- Identify Characteristic
|   |-- Manufacturer Characteristic
|   |-- Model Characteristic
|   |-- Name Characteristic
|   |-- Serial Characteristic
|   
|-- Fan Service
|   |-- On Characteristic
|   |-- Rotation Direction Characteristic
|   |-- Rotation Speed Characteristic
|
|-- Lightbulb Service
|   |-- On Characteristic
|   |-- Brightness Characteristic
|   |-- Hue Characteristic
|   |-- Saturation Characteristic
```

This crate provides a pre-built Accessory for every Service predefined by Apple. Custom Characteristics and Services can be created, assembled and used alongside the predefined ones.

For a full list of the predefined Characteristics, Services and Accessories, see the [docs](https://docs.rs/hap/) or [Apple's official specification](https://developer.apple.com/homekit/).


## Usage Examples

Creating a simple outlet Accessory and starting the IP transport:

```rust
extern crate hap;

use hap::{
    transport::{Transport, IpTransport},
    accessory::{Category, Information, outlet},
    Config,
};

fn main() {
    let info = Information {
        name: "Outlet".into(),
        ..Default::default()
    };

    let outlet = outlet::new(info).unwrap();

    let config = Config {
        name: "Outlet".into(),
        category: Category::Outlet,
        ..Default::default()
    };

    let mut ip_transport = IpTransport::new(config, vec![Box::new(outlet)]).unwrap();
    ip_transport.start().unwrap();
}
```

Using the `Readable` and `Updatable` traits to react to remote value reads and updates:

```rust
extern crate hap;

use hap::{
    transport::{Transport, IpTransport},
    accessory::{Category, Information, outlet},
    characteristic::{Readable, Updatable},
    Config,
    HapType,
};

#[derive(Clone)]
pub struct VirtualOutlet {
    on: bool,
}

impl Readable<bool> for VirtualOutlet {
    fn on_read(&mut self, _: HapType) -> Option<bool> {
        println!("On read.");
        Some(self.on)
    }
}

impl Updatable<bool> for VirtualOutlet {
    fn on_update(&mut self, old_val: &bool, new_val: &bool, _: HapType) {
        println!("On updated from {} to {}.", old_val, new_val);
        if new_val != old_val { self.on = new_val.clone(); }
    }
}

fn main() {
    let info = Information {
        name: "Outlet".into(),
        ..Default::default()
    };

    let mut outlet = outlet::new(info).unwrap();

    let virtual_outlet = VirtualOutlet { on: false };
    outlet.inner.outlet.inner.on.set_readable(virtual_outlet.clone()).unwrap();
    outlet.inner.outlet.inner.on.set_updatable(virtual_outlet).unwrap();

    let config = Config {
        name: "Outlet".into(),
        category: Category::Outlet,
        ..Default::default()
    };

    let mut ip_transport = IpTransport::new(config, vec![Box::new(outlet)]).unwrap();
    ip_transport.start().unwrap();
}
```

Setting a Characteristic value directly:

```rust
outlet.inner.outlet.inner.on.set_value(true).unwrap();
```

Change dependent Characteristics on value changes:

```rust
use std::{rc::Rc, cell::RefCell};

extern crate hap;

use hap::{
    transport::{Transport, IpTransport},
    accessory::{Category, Information, door},
    characteristic::{Characteristic, Readable, Updatable},
    Config,
    HapType,
};

pub struct VirtualDoorInner {
    current_position: u8,
    target_position: u8,
}

#[derive(Clone)]
pub struct VirtualDoor {
    inner: Rc<RefCell<VirtualDoorInner>>,
    current_position: Characteristic<u8>,
}

impl VirtualDoor {
    pub fn new(inner: VirtualDoorInner, current_position: Characteristic<u8>) -> VirtualDoor {
        VirtualDoor { inner: Rc::new(RefCell::new(inner)), current_position }
    }
}

impl Readable<u8> for VirtualDoor {
    fn on_read(&mut self, hap_type: HapType) -> Option<u8> {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Current position read.");
                Some(self.inner.borrow().current_position)
            },
            HapType::TargetPosition => {
                println!("Target position read.");
                Some(self.inner.borrow().target_position)
            },
            _ => None,
        }
    }
}

impl Updatable<u8> for VirtualDoor {
    fn on_update(&mut self, old_val: &u8, new_val: &u8, hap_type: HapType) {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Current position updated from {} to {}.", old_val, new_val);
                if new_val != old_val {
                    self.inner.borrow_mut().current_position = new_val.clone();
                }
            },
            HapType::TargetPosition => {
                println!("Target position updated from {} to {}.", old_val, new_val);
                if new_val != old_val {
                    {
                        let mut inner = self.inner.borrow_mut();
                        inner.target_position = new_val.clone();
                        inner.current_position = new_val.clone();
                    }
                    self.current_position.set_value(*new_val).unwrap();
                }
            },
            _ => {},
        }
    }
}

fn main() {
    let mut door = door::new(Information {
        name: "Door".into(),
        ..Default::default()
    }).unwrap();
    let virtual_door = VirtualDoor::new(
        VirtualDoorInner { current_position: 0, target_position: 0 },
        door.inner.door.inner.current_position.clone(),
    );
    door.inner.door.inner.current_position.set_readable(virtual_door.clone()).unwrap();
    door.inner.door.inner.current_position.set_updatable(virtual_door.clone()).unwrap();
    door.inner.door.inner.target_position.set_readable(virtual_door.clone()).unwrap();
    door.inner.door.inner.target_position.set_updatable(virtual_door).unwrap();

    let config = Config {
        name: "Door".into(),
        category: Category::Door,
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new(config, vec![Box::new(door)]).unwrap();
    ip_transport.start().unwrap();
}
```
