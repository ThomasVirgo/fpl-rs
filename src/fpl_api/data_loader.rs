use crate::fpl_api::endpoints::{get_fpl_url, FPLEndpoint};

pub struct DataLoader {
    manager_id: i32,
    event_id: i32,
}

impl DataLoader {
    fn endpoints(self) -> Vec<FPLEndpoint> {
        vec![
            FPLEndpoint::ManagerSummary {
                manager_id: self.manager_id,
            },
            FPLEndpoint::ManagerTeam {
                manager_id: self.manager_id,
                event_id: self.event_id,
            },
            FPLEndpoint::ManagerTransfers {
                manager_id: self.manager_id,
            },
            FPLEndpoint::ManagerHistory {
                manager_id: self.manager_id,
            },
        ]
    }

    fn urls(self) -> Vec<String> {
        let endpoints = self.endpoints();
        let rtn = endpoints
            .iter()
            .map(|endpoint| get_fpl_url(endpoint))
            .collect();
        rtn
    }
}
