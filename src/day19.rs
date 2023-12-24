use anyhow::{anyhow, bail, Context, Error, Result};
use std::{
    borrow::Borrow,
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::Range,
    str::FromStr,
};

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<Part> for usize {
    fn from(value: Part) -> Self {
        value.x + value.m + value.a + value.s
    }
}

impl FromStr for Part {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let trimmed_input = input.strip_prefix('{').context("missing {")?.strip_suffix('}').context("missing }")?;
        let [x, m, a, s] = trimmed_input.split(',').collect::<Vec<&str>>()[..] else {
            bail!("Could not parse part into 4 components: {input}");
        };
        Ok(Self {
            x: x.strip_prefix("x=").context("Missing x=")?.parse()?,
            m: m.strip_prefix("m=").context("Missing m=")?.parse()?,
            a: a.strip_prefix("a=").context("Missing a=")?.parse()?,
            s: s.strip_prefix("s=").context("Missing s=")?.parse()?,
        })
    }
}

enum Operator {
    LessThan,
    GreaterThan,
}

enum PartField {
    X,
    M,
    A,
    S,
}

enum Test {
    Absolute,
    Comparison { field: PartField, operator: Operator, value: usize },
}

impl FromStr for Test {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut input_chars = input.chars();
        let field = match input_chars.next().context("expected xmas, got nothing")? {
            'x' => PartField::X,
            'm' => PartField::M,
            'a' => PartField::A,
            's' => PartField::S,
            c => bail!("Expected xmas, got {c}: {input}"),
        };
        let operator = match input_chars.next().context("Expected <>, got nothing")? {
            '<' => Operator::LessThan,
            '>' => Operator::GreaterThan,
            c => bail!("Expected <>, got {c}: {input}"),
        };
        let value: usize = input_chars.collect::<String>().parse()?;
        Ok(Self::Comparison { field, operator, value })
    }
}

enum Conclusion {
    Accept,
    Reject,
    WorkFlow(String),
}

impl FromStr for Conclusion {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            wf => Ok(Self::WorkFlow(wf.to_string())),
        }
    }
}

struct Rule {
    test: Test,
    conclusion: Conclusion,
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        if let Some((test, conclusion)) = input.split_once(':') {
            Ok(Self { test: test.parse()?, conclusion: conclusion.parse()? })
        } else {
            Ok(Self { test: Test::Absolute, conclusion: input.parse()? })
        }
    }
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<&Conclusion> {
        match &self.test {
            Test::Absolute => Some(&self.conclusion),
            Test::Comparison { field, operator, value } => {
                let part_value = match field {
                    PartField::X => part.x,
                    PartField::M => part.m,
                    PartField::A => part.a,
                    PartField::S => part.s,
                };
                if match operator {
                    Operator::GreaterThan => part_value > *value,
                    Operator::LessThan => part_value < *value,
                } {
                    Some(&self.conclusion)
                } else {
                    None
                }
            }
        }
    }

    fn apply_range(&self, mut part_range: PartRange) -> Vec<(PartRange, Option<&Conclusion>)> {
        let mut conclusions: Vec<(PartRange, Option<&Conclusion>)> = Vec::new();
        match &self.test {
            Test::Absolute => conclusions.push((part_range, Some(&self.conclusion))),
            Test::Comparison { field, operator, value } => {
                let part_values = match field {
                    PartField::X => &part_range.x,
                    PartField::M => &part_range.m,
                    PartField::A => &part_range.a,
                    PartField::S => &part_range.s,
                };
                match operator {
                    Operator::GreaterThan => {
                        if part_values.contains(value) {
                            let mut first_part = part_range.clone();
                            match field {
                                PartField::X => {
                                    first_part.x.end = *value + 1;
                                    part_range.x.start = *value + 1;
                                }
                                PartField::M => {
                                    first_part.m.end = *value + 1;
                                    part_range.m.start = *value + 1;
                                }
                                PartField::A => {
                                    first_part.a.end = *value + 1;
                                    part_range.a.start = *value + 1;
                                }
                                PartField::S => {
                                    first_part.s.end = *value + 1;
                                    part_range.s.start = *value + 1;
                                }
                            };
                            conclusions.push((first_part, None));
                            conclusions.push((part_range, Some(&self.conclusion)));
                        } else if part_values.start > *value {
                            conclusions.push((part_range, Some(&self.conclusion)))
                        } else {
                            conclusions.push((part_range, None))
                        }
                    }
                    Operator::LessThan => {
                        if part_values.contains(value) {
                            let mut first_part = part_range.clone();
                            match field {
                                PartField::X => {
                                    first_part.x.end = *value;
                                    part_range.x.start = *value;
                                }
                                PartField::M => {
                                    first_part.m.end = *value;
                                    part_range.m.start = *value;
                                }
                                PartField::A => {
                                    first_part.a.end = *value;
                                    part_range.a.start = *value;
                                }
                                PartField::S => {
                                    first_part.s.end = *value;
                                    part_range.s.start = *value;
                                }
                            };
                            conclusions.push((first_part, Some(&self.conclusion)));
                            conclusions.push((part_range, None));
                        } else if part_values.end <= *value {
                            conclusions.push((part_range, Some(&self.conclusion)))
                        } else {
                            conclusions.push((part_range, None))
                        }
                    }
                }
            }
        }
        conclusions
    }
}

struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
}

impl PartialEq for WorkFlow {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for WorkFlow {}

impl Borrow<String> for WorkFlow {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl FromStr for WorkFlow {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let (name, rs) = input
            .strip_suffix('}')
            .with_context(|| format!("Missing }}: {input}"))?
            .split_once('{')
            .with_context(|| format!("Missing {{: {input}"))?;
        let mut rules: Vec<Rule> = Vec::new();
        for r in rs.split(',') {
            rules.push(r.parse()?);
        }
        Ok(Self { name: name.to_string(), rules })
    }
}

impl Hash for WorkFlow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl WorkFlow {
    fn accepts(&self, part: &Part, workflows: &HashSet<WorkFlow>) -> Result<bool> {
        for rule in self.rules.iter() {
            match rule.apply(part) {
                Some(Conclusion::Accept) => return Ok(true),
                Some(Conclusion::Reject) => return Ok(false),
                Some(Conclusion::WorkFlow(name)) => {
                    let wf = workflows.get(name).with_context(|| format!("Could not find workflow {name}"))?;
                    return wf.accepts(part, workflows);
                }
                None => {}
            }
        }
        Err(anyhow!("no conclusion for workflow {} on part {:?}", self.name, part))
    }

    fn accepts_range(&self, part_range: PartRange, workflows: &HashSet<WorkFlow>) -> Result<Vec<PartRange>> {
        let mut accepted: Vec<PartRange> = Vec::new();
        let mut part_ranges = vec![part_range];
        for rule in self.rules.iter() {
            let mut remaining_part_ranges: Vec<PartRange> = Vec::new();
            for part_range in part_ranges {
                for (range, conclusion) in rule.apply_range(part_range) {
                    match conclusion {
                        None => remaining_part_ranges.push(range),
                        Some(Conclusion::Accept) => accepted.push(range),
                        Some(Conclusion::Reject) => {}
                        Some(Conclusion::WorkFlow(name)) => {
                            let wf = workflows.get(name).with_context(|| format!("Could not find workflow {name}"))?;
                            accepted.extend(wf.accepts_range(range, workflows)?);
                        }
                    }
                }
            }
            part_ranges = remaining_part_ranges;
        }
        Ok(accepted)
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let mut workflows: HashSet<WorkFlow> = HashSet::default();

    let (ws, ps) = input.split_once("\n\n").context("Could not split input into workflows and parts")?;

    for wf in ws.lines() {
        workflows.insert(wf.parse()?);
    }

    let mut parts: Vec<Part> = Vec::default();
    for p in ps.lines() {
        parts.push(p.parse()?);
    }

    let start: &WorkFlow = workflows.get(&"in".to_string()).context("Could not find starting workflow")?;

    let mut value_of_accepted_parts = 0;
    for part in parts {
        if start.accepts(&part, &workflows)? {
            value_of_accepted_parts += usize::from(part);
        }
    }

    Ok(value_of_accepted_parts)
}

#[derive(Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl Default for PartRange {
    fn default() -> Self {
        Self { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 }
    }
}

impl From<PartRange> for usize {
    fn from(value: PartRange) -> Self {
        value.x.len() * value.m.len() * value.a.len() * value.s.len()
    }
}

pub fn part2(input: &str) -> Result<usize> {
    let mut workflows: HashSet<WorkFlow> = HashSet::default();
    for wf in input.lines() {
        if wf.is_empty() {
            break;
        }
        workflows.insert(wf.parse()?);
    }
    let start: &WorkFlow = workflows.get(&"in".to_string()).context("Could not find starting workflow")?;
    let accepted_ranges = start.accepts_range(PartRange::default(), &workflows)?;
    Ok(accepted_ranges.into_iter().map(usize::from).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 19114);
        assert_eq!(part2(EXAMPLE).unwrap(), 167409079868000);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 432788);
        assert_eq!(part2(&input).unwrap(), 142863718918201);
    }
}
