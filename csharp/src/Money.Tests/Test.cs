namespace Money.Tests;

using NUnit.Framework;

using Money;

public class Test
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestAddMultiplication()
    {
        Dollar five = new Dollar(5);
        Assert.True(five.Times(2).Equals(new Dollar(10)));
        Assert.True(five.Times(3).Equals(new Dollar(15)));
    }

    [Test]
    public void TestEquality()
    {
        Assert.True(new Dollar(5).Equals(new Dollar(5)));
    }

    [Test]
    public void TestFrancMultiplication()
    {
        Franc five = new Franc(5);
        Assert.True(five.Times(2).Equals(new Franc(10)));
        Assert.True(five.Times(3).Equals(new Franc(15)));
    }

}
