use compositor::Shell;
use wlroots::{Origin, SurfaceHandle};
use wlroots::XdgV6ShellState::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct View {
    pub shell: Shell,
    pub origin: Origin
}

impl View {
    pub fn new(shell: Shell) -> View {
        View { shell,
               origin: Origin::default() }
    }

    pub fn surface(&mut self) -> SurfaceHandle {
        match &mut self.shell {
            &mut Shell::XdgV6(ref mut xdg_surface) => {
                with_handles!([(xdg_surface: {xdg_surface})] => {
                    xdg_surface.surface()
                }).unwrap()
            }
        }
    }

    pub fn activate(&mut self, activate: bool) {
        match &mut self.shell {
            &mut Shell::XdgV6(ref mut xdg_surface) => {
                with_handles!([(xdg_surface: {xdg_surface})] => {
                    match xdg_surface.state() {
                        Some(&mut TopLevel(ref mut toplevel)) => {
                            toplevel.set_activated(activate);
                        },
                        _ => unimplemented!()
                    }
                }).unwrap();
            }
        }
    }

    pub fn for_each_surface(&mut self, f: &mut FnMut(SurfaceHandle, i32, i32)) {
        match &mut self.shell {
            &mut Shell::XdgV6(ref mut xdg_surface) => {
                with_handles!([(xdg_surface: {xdg_surface})] => {
                    xdg_surface.for_each_surface(f);
                }).unwrap();
            }
        }
    }
}