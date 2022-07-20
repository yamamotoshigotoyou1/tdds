namespace Money.Tests;

using NUnit.Framework;

public class Test
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestAddMultiplication()
    {
        Money five = Money.Dollar(5);
        Assert.True(five.Times(2).Equals(Money.Dollar(10)));
        Assert.True(five.Times(3).Equals(Money.Dollar(15)));
    }

    [Test]
    public void TestEquality()
    {
        Assert.True(Money.Dollar(5).Equals(Money.Dollar(5)));
        Assert.False(Money.Dollar(5).Equals(Money.Dollar(6)));
        Assert.False(Money.Franc(5).Equals(Money.Dollar(5)));
    }

    [Test]
    public void TestCurrency()
    {
        Assert.That(Money.Dollar(1).Currency(), Is.EqualTo("USD"));
        Assert.That(Money.Franc(1).Currency(), Is.EqualTo("CHF"));
    }

    [Test]
    public void TestSimpleAddition()
    {
        Money five = Money.Dollar(5);
        Expression sum = five.Plus(five);
        Bank bank = new Bank();
        Money reduced = bank.Reduce(sum, "USD");
        Assert.True(Money.Dollar(10).Equals(reduced));
    }
}
