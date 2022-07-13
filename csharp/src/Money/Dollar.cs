namespace Money;

public class Dollar : Money
{

    public Dollar(int amount, string currency):base(amount, currency)
    {
        this.currency = currency;
        this.amount = amount;
    }

}
