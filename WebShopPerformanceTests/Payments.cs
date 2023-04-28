namespace WebShopPerformanceTests;
using ServiceReference;

public class Tests
{
    String username = Environment.GetEnvironmentVariable("API_USERNAME") ?? "";
    String password = Environment.GetEnvironmentVariable("API_PASSWORD") ?? "";

    [Test]
    public async Task TestGetPaymentMethodCollectionAsync()
    {
        NopServiceClient service = new NopServiceClient();
        
        // Settings
        int maxTime = 120;
        int timeout = 20000;

        // Test service with performancetester
        PerformanceTester.PerformanceResult result = await PerformanceTester.testLinear(maxTime, timeout, async Task<bool> () => {
            bool isSuccessFull = true;

            try {
                await service.GetPaymentMethodCollectionAsync(username, password);
            } catch (Exception e) {
                isSuccessFull = false;
            }

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