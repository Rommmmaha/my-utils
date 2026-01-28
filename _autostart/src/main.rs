use hyprland::data::Clients;
use hyprland::dispatch::*;
use hyprland::prelude::*;
use std::{thread, time::Duration};
fn main() -> anyhow::Result<()> {
  loop {
    let clients = Clients::get()?;
    let vivaldi = clients.iter().find(|c| c.class == "vivaldi-snapshot");
    if let Some(client) = vivaldi {
      let target = WorkspaceIdentifierWithSpecial::Id(3);
      let window = WindowIdentifier::Address(client.address.clone());
      Dispatch::call(DispatchType::MoveToWorkspaceSilent(target, Some(window)))?;
      break;
    }
    thread::sleep(Duration::from_millis(500));
  }
  Ok(())
}
