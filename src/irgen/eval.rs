use super::scopes::Scopes;
use super::values::Value;
use crate::ast::*;

/// Trait for evaluating constant.
pub trait Evaluate {
    fn eval(&self, scopes: &Scopes) -> Option<i32>;
}

impl Evaluate for Expr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        self.lor.eval(scopes)
    }
}

impl Evaluate for LVal {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        let val = scopes.value(&self.id.name).ok()?;
        if self.indices.is_empty() {
            match val {
                Value::Const(i) => Some(*i),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Evaluate for PrimaryExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Expr(exp) => exp.eval(scopes),
            Self::LVal(lval) => lval.eval(scopes),
            Self::Number(num) => Some(num.value),
        }
    }
}

impl Evaluate for UnaryExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Primary(primary) => primary.eval(scopes),
            Self::Call(_) => None,
            Self::Unary(op, exp) => exp.eval(scopes).map(|exp| match op {
                UnaryOp::Neg(_) => -exp,
                UnaryOp::LNot(_) => (exp == 0) as i32,
            }),
        }
    }
}

impl Evaluate for MulExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Unary(exp) => exp.eval(scopes),
            Self::MulUnary(lhs, op, rhs) => match (lhs.eval(scopes), rhs.eval(scopes)) {
                (Some(lhs), Some(rhs)) => match op {
                    MulOp::Mul(_) => Some(lhs * rhs),
                    MulOp::Div(_) => (rhs != 0).then_some(lhs / rhs),
                    MulOp::Mod(_) => (rhs != 0).then_some(lhs % rhs),
                },
                _ => None,
            },
        }
    }
}

impl Evaluate for AddExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Mul(exp) => exp.eval(scopes),
            Self::AddMul(lhs, op, rhs) => match (lhs.eval(scopes), rhs.eval(scopes)) {
                (Some(lhs), Some(rhs)) => Some(match op {
                    AddOp::Add(_) => lhs + rhs,
                    AddOp::Sub(_) => lhs - rhs,
                }),
                _ => None,
            },
        }
    }
}

impl Evaluate for RelExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Add(exp) => exp.eval(scopes),
            Self::RelAdd(lhs, op, rhs) => match (lhs.eval(scopes), rhs.eval(scopes)) {
                (Some(lhs), Some(rhs)) => Some(match op {
                    RelOp::Lt(_) => (lhs < rhs) as i32,
                    RelOp::Gt(_) => (lhs > rhs) as i32,
                    RelOp::Le(_) => (lhs <= rhs) as i32,
                    RelOp::Ge(_) => (lhs >= rhs) as i32,
                }),
                _ => None,
            },
        }
    }
}

impl Evaluate for EqExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Rel(exp) => exp.eval(scopes),
            Self::EqRel(lhs, op, rhs) => match (lhs.eval(scopes), rhs.eval(scopes)) {
                (Some(lhs), Some(rhs)) => Some(match op {
                    EqOp::Eq(_) => (lhs == rhs) as i32,
                    EqOp::Ne(_) => (lhs != rhs) as i32,
                }),
                _ => None,
            },
        }
    }
}

impl Evaluate for LAndExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::Eq(exp) => exp.eval(scopes),
            Self::LAndEq(lhs, rhs) => match (lhs.eval(scopes), rhs.eval(scopes)) {
                (Some(lhs), Some(rhs)) => Some((lhs != 0 && rhs != 0) as i32),
                _ => None,
            },
        }
    }
}

impl Evaluate for LOrExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        match self {
            Self::LAnd(exp) => exp.eval(scopes),
            Self::LOrLAnd(lhs, rhs) => match (lhs.eval(scopes), rhs.eval(scopes)) {
                (Some(lhs), Some(rhs)) => Some((lhs != 0 || rhs != 0) as i32),
                _ => None,
            },
        }
    }
}

impl Evaluate for ConstExpr {
    fn eval(&self, scopes: &Scopes) -> Option<i32> {
        self.exp.eval(scopes)
    }
}
