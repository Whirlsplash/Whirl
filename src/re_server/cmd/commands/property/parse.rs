// Copyleft 2021-2021 Whirlsplash
// SPDX-License-Identifier: GPL-3.0-only

use crate::re_server::net::structure::NetworkProperty;

pub fn find_property_in_property_list(
  property_list: &[NetworkProperty],
  property: i32,
) -> &NetworkProperty {
  property_list
    .iter()
    .find(|i| i.prop_id == property)
    .unwrap()
}
