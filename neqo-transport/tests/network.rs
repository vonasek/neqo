// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "deny-warnings", deny(warnings))]
#![warn(clippy::pedantic)]

#[macro_use]
mod sim;

use neqo_transport::{ConnectionError, Error, State};
use sim::{
    connection::{ConnectionNode, ReachState, ReceiveData, SendData},
    network::{Delay, Drop},
    Simulator,
};
use std::ops::Range;
use std::time::Duration;

// Some constants that are useful common values.
const TRANSFER_AMOUNT: usize = 1 << 20;
const ZERO: Duration = Duration::from_millis(0);
const DELAY: Duration = Duration::from_millis(50);
const DELAY_RANGE: Range<Duration> = DELAY..Duration::from_millis(55);

simulate!(
    connect_direct,
    [
        ConnectionNode::new_client(boxed![ReachState::new(State::Confirmed)]),
        ConnectionNode::new_server(boxed![ReachState::new(State::Confirmed)]),
    ]
);

simulate!(
    idle_timeout,
    [
        ConnectionNode::new_client(boxed![
            ReachState::new(State::Confirmed),
            ReachState::new(State::Closed(ConnectionError::Transport(
                Error::IdleTimeout
            )))
        ]),
        ConnectionNode::new_server(boxed![
            ReachState::new(State::Confirmed),
            ReachState::new(State::Closed(ConnectionError::Transport(
                Error::IdleTimeout
            )))
        ]),
    ]
);

simulate!(
    transfer,
    [
        ConnectionNode::new_client(boxed![SendData::new(TRANSFER_AMOUNT)]),
        ConnectionNode::new_server(boxed![ReceiveData::new(TRANSFER_AMOUNT)]),
    ]
);

simulate!(
    connect_fixed_rtt,
    [
        ConnectionNode::new_client(boxed![ReachState::new(State::Confirmed)]),
        Delay::new(DELAY..DELAY),
        ConnectionNode::new_server(boxed![ReachState::new(State::Confirmed)]),
        Delay::new(DELAY..DELAY),
    ],
);

simulate!(
    transfer_delay_drop,
    [
        ConnectionNode::new_client(boxed![SendData::new(TRANSFER_AMOUNT)]),
        Delay::new(DELAY_RANGE),
        Drop::percentage(1),
        ConnectionNode::new_server(boxed![ReceiveData::new(TRANSFER_AMOUNT)]),
        Delay::new(DELAY_RANGE),
        Drop::percentage(1),
    ],
);

#[test]
fn transfer_fixed_seed() {
    let mut sim = Simulator::new(
        "transfer_fixed_seed",
        boxed![
            ConnectionNode::new_client(boxed![SendData::new(TRANSFER_AMOUNT)]),
            Delay::new(ZERO..DELAY),
            Drop::percentage(1),
            ConnectionNode::new_server(boxed![ReceiveData::new(TRANSFER_AMOUNT)]),
            Delay::new(ZERO..DELAY),
            Drop::percentage(1),
        ],
    );
    sim.seed_str("117f65d90ee5c1a7fb685f3af502c7730ba5d31866b758d98f5e3c2117cf9b86");
    sim.run();
}
