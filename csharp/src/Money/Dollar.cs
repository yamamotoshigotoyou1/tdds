namespace Money;

public class Dollar
{
    public int amount;

    public Dollar(int amount)
    {
        this.amount = amount;
    }

    public void Times(int multiplier)
    {
        this.amount *= multiplier;
    }
}
