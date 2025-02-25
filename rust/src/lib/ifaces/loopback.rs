// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{BaseInterface, ErrorKind, InterfaceType, NmstateError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
/// Loopback interface. Only contain information of [BaseInterface].
/// Limitations
///  * Cannot enable DHCP or autoconf.
///  * The [InterfaceState::Absent] can only restore the loopback configure back
///    to default.
///  * Ignore the request of disable IPv4 or IPv6.
///  * Even not desired, the `127.0.0.1/8` and `::1` are always appended to
///    static IP address list.
///  * Require NetworkManager 1.41+ unless in kernel only mode.
/// Example yaml outpuf of `[crate::NetworkState]` with loopback interface:
/// ```yml
/// interfaces:
/// - name: lo
///   type: loopback
///   state: up
///   mtu: 65535
///   ipv4:
///     enabled: true
///     address:
///     - ip: 127.0.0.1
///       prefix-length: 8
///   ipv6:
///     enabled: true
///     address:
///     - ip: ::1
///       prefix-length: 128
///   accept-all-mac-addresses: false
/// ```
pub struct LoopbackInterface {
    #[serde(flatten)]
    pub base: BaseInterface,
}

impl Default for LoopbackInterface {
    fn default() -> Self {
        let mut base = BaseInterface::new();
        base.iface_type = InterfaceType::Loopback;
        Self { base }
    }
}

impl LoopbackInterface {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn sanitize(&self) -> Result<(), NmstateError> {
        if self.base.ipv4.as_ref().map(|i| i.enabled) == Some(false) {
            return Err(NmstateError::new(
                ErrorKind::InvalidArgument,
                "Loopback interface cannot be have IPv4 disabled".to_string(),
            ));
        }
        if self.base.ipv6.as_ref().map(|i| i.enabled) == Some(false) {
            return Err(NmstateError::new(
                ErrorKind::InvalidArgument,
                "Loopback interface cannot be have IPv6 disabled".to_string(),
            ));
        }
        if self.base.ipv4.as_ref().map(|i| i.is_auto()) == Some(true) {
            return Err(NmstateError::new(
                ErrorKind::InvalidArgument,
                "Loopback interface cannot be have IPv4 DHCP enabled"
                    .to_string(),
            ));
        }
        if self.base.ipv6.as_ref().map(|i| i.is_auto()) == Some(true) {
            return Err(NmstateError::new(
                ErrorKind::InvalidArgument,
                "Loopback interface cannot be have IPv6 \
                autoconf/DHCPv6 enabled"
                    .to_string(),
            ));
        }
        Ok(())
    }
}
