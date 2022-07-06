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
        Assert.True(Money.Franc(5).Equals(Money.Franc(5)));
        Assert.False(Money.Franc(5).Equals(Money.Franc(6)));
        Assert.False(Money.Franc(5).Equals(Money.Dollar(5)));
    }

    [Test]
    public void TestFrancMultiplication()
    {
        Money five = Money.Franc(5);
        Assert.True(five.Times(2).Equals(Money.Franc(10)));
        Assert.True(five.Times(3).Equals(Money.Franc(15)));
    }

}
