namespace WebShopPerformanceTests;
using ServiceReference;

public class Tests
{
    String username = Environment.GetEnvironmentVariable("API_USERNAME") ?? "jbtwaalf@gmail.com";
    String password = Environment.GetEnvironmentVariable("API_PASSWORD") ?? "Start1234%";

    [Test]
    public async Task TestGetPaymentMethodCollectionAsync()
    {
        int maxTime = 120;
        int timeout = 20000;
        NopServiceClient service = new NopServiceClient();

        PerformanceTester.PerformanceResult result = await PerformanceTester.testLinear(maxTime, timeout, async Task<bool> () => {
            bool isSuccessFull = true;

            try {
                await service.GetPaymentMethodCollectionAsync(username, password);
            } catch (Exception e) {
                isSuccessFull = false;
            }

            Task.Delay(200);

            return isSuccessFull;
        });

        // Test all successfull
        Console.WriteLine("Amount successfull: " + result.amountSuccessFull);
        Assert.IsTrue(result.amountFailed == 0);

        // Test average response time
        Console.WriteLine("Average response time: " + result.averageResponseTime.TotalMilliseconds);
        Assert.IsTrue(result.averageResponseTime.TotalMilliseconds < 300);

        // Warn if average response time is above 0.2
        if (result.averageResponseTime.TotalMilliseconds > 200) {
            Console.WriteLine("WARN: Average response time is above 0.2 seconds");
        }
    }
}