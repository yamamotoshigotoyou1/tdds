namespace Money;

public class Franc : Money
{
    public Franc(int amount, string currency) : base(amount, currency)
    {
        this.amount = amount;
        this.currency = "CHF";
    }
}
