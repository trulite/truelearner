use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Arrow {
    source: String,
    label: String,
    target: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BehaviorDiagram {
    name: String,
    arrows: Vec<(String, Arrow)>,
}

impl BehaviorDiagram {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arrows: Vec::new(),
        }
    }

    pub fn arrow(
        &mut self,
        name: impl Into<String>,
        source: impl Into<String>,
        label: impl Into<String>,
        target: impl Into<String>,
    ) {
        let name = name.into();
        assert!(
            !self.arrows.iter().any(|(existing, _)| existing == &name),
            "duplicate diagram arrow {name}"
        );
        self.arrows.push((
            name.clone(),
            Arrow {
                source: source.into(),
                label: label.into(),
                target: target.into(),
            },
        ));
    }

    #[track_caller]
    pub fn assert_commutes(&self, left: &[&str], right: &[&str]) {
        let left_endpoints = self.compose(left);
        let right_endpoints = self.compose(right);
        assert_eq!(
            left_endpoints, right_endpoints,
            "non-commuting behavior diagram:\n{self}"
        );
    }

    fn compose(&self, path: &[&str]) -> (String, String) {
        let first = self
            .arrows
            .iter()
            .find(|(name, _)| name == path.first().expect("diagram path must contain an arrow"))
            .map(|(_, arrow)| arrow)
            .unwrap_or_else(|| panic!("unknown diagram arrow in {}", self.name));
        let mut target = first.target.clone();
        for name in &path[1..] {
            let arrow = self
                .arrows
                .iter()
                .find(|(candidate, _)| candidate == name)
                .map(|(_, arrow)| arrow)
                .unwrap_or_else(|| panic!("unknown diagram arrow {name} in {}", self.name));
            assert_eq!(
                target, arrow.source,
                "disconnected behavior path at {name}:\n{self}"
            );
            target.clone_from(&arrow.target);
        }
        (first.source.clone(), target)
    }
}

impl fmt::Display for BehaviorDiagram {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "{}", self.name)?;
        for (name, arrow) in &self.arrows {
            writeln!(
                formatter,
                "  [{}] --{}: {}--> [{}]",
                arrow.source, name, arrow.label, arrow.target
            )?;
        }
        Ok(())
    }
}
