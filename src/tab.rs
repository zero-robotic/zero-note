
use druid::im::Vector;
use druid::widget::{Axis, TabsEdge, TabsTransition};
use druid::{Data, Lens};


#[derive(Data, Clone, Lens)]
pub struct TabConfig {
    axis: Axis,
    edge: TabsEdge,
    transition: TabsTransition,
}

impl Default for TabConfig {
    fn default() -> Self {
        TabConfig {
            axis: Axis::Horizontal,
            edge: TabsEdge::Leading,
            transition: Default::default(),
        }
    }
}

#[derive(Data, Clone, Lens)]
pub struct DynamicTabData {
    highest_tab: usize,
    removed_tabs: usize,
    tab_labels: Vector<usize>,
}

impl DynamicTabData {
    pub fn new(highest_tab: usize) -> Self {
        DynamicTabData {
            highest_tab,
            removed_tabs: 0,
            tab_labels: (1..=highest_tab).collect(),
        }
    }

    pub fn add_tab(&mut self) {
        self.highest_tab += 1;
        self.tab_labels.push_back(self.highest_tab)
    }

    pub fn remove_tab(&mut self, idx: usize) {
        if idx >= self.tab_labels.len() {
            tracing::warn!("Attempt to remove non existent tab at index {}", idx)
        } else {
            self.removed_tabs += 1;
            self.tab_labels.remove(idx);
        }
    }

    // This provides a key that will monotonically increase as interactions occur.
    pub fn tabs_key(&self) -> (usize, usize) {
        (self.highest_tab, self.removed_tabs)
    }
}
