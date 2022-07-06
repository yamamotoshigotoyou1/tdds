namespace Money;

public class Dollar : Money
{
    public Dollar(int amount)
    {
        this.amount = amount;
    }

    public override Money Times(int multiplier)
    {
        return new Dollar(this.amount * multiplier);
    }
}
