namespace Money;

public class Franc
{
    public int amount;

    public Franc(int amount)
    {
        this.amount = amount;
    }

    public Franc Times(int multiplier)
    {
        return new Franc(this.amount * multiplier);
    }

    public bool Equals(Franc franc)
    {
        return this.amount == franc.amount;
    }
}
