namespace WebShopUnitTests;
public class Tests
{
    [Test]
    public void TestDoNotCalculateShippingCosts()
    {
        Double costs = CostsCalculator.ShippingCosts(false, "Ground", 1400);
        Assert.That(costs, Is.EqualTo(0));
    }

    [Test]
    public void TestCalculateShippingCosts()
    {
        Double costs = CostsCalculator.ShippingCosts(true, "nonsensestring", 1400);
        Assert.That(costs, Is.EqualTo(0));
    }

    [Test]
    public void TestCalculateShippingCostsHigherThan1500()
    {
        Double costs = CostsCalculator.ShippingCosts(true, "nonsensestring", 2700);
        Assert.That(costs, Is.EqualTo(0));
    }

    [Test]
    public void TestGroundShipping()
    {
        Double costs = CostsCalculator.ShippingCosts(true, "Ground", 1501);
        Assert.That(costs, Is.EqualTo(0));

        costs = CostsCalculator.ShippingCosts(true, "Ground", 1400);
        Assert.That(costs, Is.EqualTo(100));
    }

    [Test]
    public void TestInStoreShipping()
    {
        Double costs = CostsCalculator.ShippingCosts(true, "InStore", 1501);
        Assert.That(costs, Is.EqualTo(0));

        costs = CostsCalculator.ShippingCosts(true, "InStore", 1400);
        Assert.That(costs, Is.EqualTo(50));
    }

    [Test]
    public void TestNextDayAirShipping()
    {
        Double costs = CostsCalculator.ShippingCosts(true, "NextDayAir", 1501);
        Assert.That(costs, Is.EqualTo(0));

        costs = CostsCalculator.ShippingCosts(true, "NextDayAir", 1400);
        Assert.That(costs, Is.EqualTo(250));
    }

    [Test]
    public void TestSecondDayAirShipping()
    {
        Double costs = CostsCalculator.ShippingCosts(true, "SecondDayAir", 1501);
        Assert.That(costs, Is.EqualTo(0));

        costs = CostsCalculator.ShippingCosts(true, "SecondDayAir", 1400);
        Assert.That(costs, Is.EqualTo(125));
    }
}
