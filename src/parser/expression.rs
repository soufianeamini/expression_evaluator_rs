use super::Expression;

pub struct Literal {
    pub value: f64,
}

pub struct Binary {
    pub operator: char,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Literal {
    fn evaluate(&self) -> f64 {
        self.value
    }
}

impl Expression for Binary {
    fn evaluate(&self) -> f64 {
        match self.operator {
            '+' => self.left.evaluate() + self.right.evaluate(),
            '-' => self.left.evaluate() - self.right.evaluate(),
            '*' => self.left.evaluate() * self.right.evaluate(),
            '/' => self.left.evaluate() / self.right.evaluate(),
            _ => 0.,
        }
    }
}

