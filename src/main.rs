use std::ops;

#[derive(Debug)]
#[derive(Clone)]
pub enum Currency {
  Dollar,
  Baht,
  Bitcoin,
}

impl Currency {
  // The amount of dollar per one bucks in that currency
  fn rate_to_dollar(&self) -> f32 {
    match *self {
      Currency::Dollar => 1.0,
      Currency::Baht => 0.031,
      Currency::Bitcoin => 19519.0
    }
  }

  // The amount of currency a dollar can buy
  fn rate_from_dollar(&self) -> f32 {
    match *self {
      Currency::Dollar => 1.0,
      Currency::Baht => 32.51,
      Currency::Bitcoin => 0.000051
    }
  }
}

#[derive(Debug)]
struct Cash {
  currency: Currency,
  amount: f32,
}

impl Cash {
  pub fn new(amount: f32, currency: Currency) -> Cash {
    Cash { amount, currency }
  }

  fn convert(&self, currency: Currency) -> Cash {
    // Converts our currency to dollar
    let amount_in_dollar = self.amount * self.currency.rate_to_dollar();

    // Convert the dollar to our target currency
    let amount_in_currency = amount_in_dollar * currency.rate_from_dollar();

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
  let my_cash = Cash::new(1.0, Currency::Bitcoin);
  let your_cash = Cash::new(30000.0, Currency::Baht);
  let total_cash = my_cash + your_cash;

  println!("{:?}", total_cash);
}
