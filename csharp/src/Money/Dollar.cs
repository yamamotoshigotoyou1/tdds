namespace Money;

public class Dollar
{
    public int amount;

    public Dollar(int amount)
    {
        this.amount = amount;
    }

    public Dollar Times(int multiplier)
    {
        return new Dollar(this.amount * multiplier);
    }

    public bool Equals(Dollar dollar)
    {
        return this.amount == dollar.amount;
    }
}
