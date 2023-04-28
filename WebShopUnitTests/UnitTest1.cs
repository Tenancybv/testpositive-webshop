namespace WebShopUnitTests;
public class Tests
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void Test1()
    {
        //TODO: implement actual test
        Assert.IsNotNull(CostsCalculator.ShippingCosts(true, "InStore", 10));
    }
}