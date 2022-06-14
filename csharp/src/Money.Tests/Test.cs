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
        var five = new Dollar(5);
        five.Times(2);
        Assert.That(five.amount, Is.EqualTo(10));
    }
}
