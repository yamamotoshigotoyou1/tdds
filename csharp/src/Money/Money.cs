using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Money
{
    public class Money : Expression
    {
        protected string currency;
        public int amount;

        public Money(int amount, string currency)
        {
            this.currency = currency;
            this.amount = amount;
        }

        public Money Times(int multiplier)
        {
            return new Money(this.amount * multiplier, currency);
        }
        public string Currency()
        {
            return currency!;
        }

        public bool Equals(Money money)
        {
            return this.amount == money.amount && Currency().Equals(money.Currency());
        }

        public static Money Dollar(int amount)
        {
            return new Money(amount, "USD");
        }
        public static Money Franc(int amount)
        {
            return new Money(amount, "CHF");
        }

        public override string ToString()
        {
            return amount + " " + currency;
        }

        public Expression Plus(Money addend)
        {
            return new Money(amount + addend.amount, this.currency);
        }
    }
}
