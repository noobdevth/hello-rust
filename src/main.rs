use std::ops;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Currency {
  name: &'static str,
  rate_to_dollar: f32,
  rate_from_dollar: f32,
}

impl fmt::Display for Currency {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

#[derive(Debug)]
struct Cash {
  currency: Currency,
  amount: f32,
}

impl fmt::Display for Cash {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {}", self.amount, self.currency)
  }
}

pub const USD: Currency = Currency {
  name: "USD",
  rate_to_dollar: 1.0,
  rate_from_dollar: 1.0,
};

pub const THB: Currency = Currency {
  name: "THB",
  rate_to_dollar: 0.031,
  rate_from_dollar: 32.51,
};

pub const BTC: Currency = Currency {
  name: "BTC",
  rate_to_dollar: 19519.0,
  rate_from_dollar: 0.000051,
};

impl Cash {
  pub fn new(amount: f32, currency: Currency) -> Cash {
    Cash { amount, currency }
  }

  fn convert(&self, currency: Currency) -> Cash {
    // Converts our currency to dollar
    let amount_in_dollar = self.amount * self.currency.rate_to_dollar;

    // Convert the dollar to our target currency
    let amount_in_currency = amount_in_dollar * currency.rate_from_dollar;

    Cash::new(amount_in_currency, currency)
  }
}

impl ops::Add<Cash> for Cash {
  type Output = Cash;

  fn add(self, rhs: Cash) -> Cash {
    Cash {
      currency: self.currency.clone(),
      amount: self.amount + rhs.convert(self.currency).amount,
    }
  }
}

fn main() {
  let my_cash = Cash::new(1.0, BTC);
  let your_cash = Cash::new(30000.0, THB);
  let total_cash = my_cash + your_cash;

  println!("I have {}", total_cash.convert(USD));
}
