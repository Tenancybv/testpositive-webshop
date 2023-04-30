namespace WebShopPerformanceTests;
using ServiceReference;
using static WebShopPerformanceTests.PerformanceTester;

public class Tests
{
  // Settings
  String username = Environment.GetEnvironmentVariable("API_USERNAME") ?? "jbtwaalf@gmail.com";
  String password = Environment.GetEnvironmentVariable("API_PASSWORD") ?? "Start1234%";

  static int maxTime = 120;
  static int timeout = 20;
  static int maxConcurrent = 5;

  static int maxAverageResponseTime = 300;

  static int warnAverageResponseTime = 200;

  public async Task<bool> TestGetPaymentMethods()
  {
    NopServiceClient service = new NopServiceClient();
    bool isSuccessFull = true;

    try
    {
      await service.GetPaymentMethodCollectionAsync(username, password);
    }
    catch (Exception e)
    {
      isSuccessFull = false;
    }

    return isSuccessFull;
  }

  [Test]
  public async Task Test_a_GetPaymentMethodsLinear()
  {
    // Test service with performancetester and linear
    PerformanceResult result = await PerformanceTester.testLinear(maxTime, timeout, async Task<bool> () =>
    {
      return await TestGetPaymentMethods();
    });

    // Test all successfull
    Console.WriteLine("[linear] Amount successfull: " + result.amountSuccessFull);
    Assert.Zero(result.amountFailed);


    // Test average response time
    Console.WriteLine("[linear] Average response time: " + result.averageResponseTime.TotalMilliseconds);
    Assert.Less(result.averageResponseTime.TotalMilliseconds, maxAverageResponseTime);

    // Warn if average response time is above 0.2
    if (result.averageResponseTime.TotalMilliseconds > warnAverageResponseTime)
    {
      Console.WriteLine("[linear] WARN: Average response time is above 0.2 seconds");
    }
  }

  [Test]
  public async Task Test_b_GetPaymentMethodsCascading()
  {
    // Test service with performancetester and cascading
    PerformanceTester.PerformanceResult result = await PerformanceTester.testCascading(maxTime, timeout, maxConcurrent, async Task<bool> () =>
    {
      return await TestGetPaymentMethods();
    });

    // Test all successfull
    Console.WriteLine("[cascading] Amount successfull: " + result.amountSuccessFull);
    Assert.Zero(result.amountFailed);

    // Test average response time
    Console.WriteLine("[cascading] Average response time: " + result.averageResponseTime.TotalMilliseconds);
    Assert.Less(result.averageResponseTime.TotalMilliseconds, maxAverageResponseTime);

    // Warn if average response time is above 0.2
    if (result.averageResponseTime.TotalMilliseconds > warnAverageResponseTime)
    {
      Console.WriteLine("[cascading] WARN: Average response time is above 0.2 seconds");
    }
  }
}