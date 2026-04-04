use std::sync::Arc;

use derive_new::new;
use egui::{Painter, Pos2, Stroke, Style, Ui};
use hexgeo::Axials;
use hexgeo::geom::DHO;

use crate::projector::Projector;

/// Handle to interact with `egui` scoped to a specific hex
#[derive(new)]
pub struct HexUi<'a> {
    ui: &'a mut Ui,
    p: &'a Projector,
    dho: DHO,
    ax: Axials,
}

impl<'a> HexUi<'a> {
    pub fn style(&self) -> &Arc<Style> {
        self.ui.style()
    }

    pub fn orientation(&self) -> DHO {
        self.dho
    }

    pub fn axials(&self) -> Axials {
        self.ax
    }

    pub fn line<P>(&self, pts: [P; 2], stroke: Stroke)
    where
        P: Into<Pos2>,
    {
        let screenpts = pts.map(|loc| self.local_to_screen(loc));

        self.painter().line_segment(screenpts, stroke);
    }

    pub fn path<I, P>(&self, pts: I, stroke: Stroke, closed: bool)
    where
        I: IntoIterator<Item = P>,
        P: Into<Pos2>,
    {
        let mut first = None;
        let mut prev = None;
        for pt in pts {
            let pt = pt.into();
            first.get_or_insert(pt);
            if let Some(pp) = prev {
                self.line([pp, pt], stroke);
            }
            prev = Some(pt);
        }

        if let (true, Some(last), Some(first)) = (closed, prev, first) {
            self.line([last, first], stroke);
        }
    }

    fn painter(&self) -> &Painter {
        self.ui.painter()
    }

    fn local_to_screen<P>(&self, pt: P) -> Pos2
    where
        P: Into<Pos2>,
    {
        // Translate to rim-logical pos:
        let rimpos = pt.into() + self.ax.origin_to_center(self.dho);
        // Project into screen coordinates:
        self.p.project(rimpos)
    }
}
