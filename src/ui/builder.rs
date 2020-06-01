use futures::sync::mpsc::{ unbounded, UnboundedSender, UnboundedReceiver };
use xrl::{ Client, FrontendBuilder };

use super::{ XiTermService, CoreEvent };

pub struct XiTermServiceBuilder(UnboundedSender<CoreEvent>);

impl XiTermServiceBuilder {
    pub fn new() -> (Self, UnboundedReceiver<CoreEvent>) {
        let (tx, rx) = unbounded();
        (XiTermServiceBuilder(tx), rx)
    }
}

impl FrontendBuilder for XiTermServiceBuilder {
    type Frontend = XiTermService;
    fn build(self, _client: Client) -> Self::Frontend {
        XiTermService(self.0)
    }
}
