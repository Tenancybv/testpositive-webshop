namespace WebShopPerformanceTests;

public static class PerformanceTester {

  public partial class PerformanceResult {
    public TimeSpan averageResponseTime = TimeSpan.Zero;
    public int amountFailed = 0;

    public int amountSuccessFull = 0;
  }

  /*
    Tests a service by sending requests with a timeout over a duration.
    Returns a PerformanceResult with the average response time, amount of failed requests and amount of successfull requests.
  */
  public static async Task<PerformanceResult> testLinear(int durationInSeconds, int timeoutInMilliSeconds, Func<Task<bool>> test) {
    DateTime startDate = DateTime.Now;

    // Initialize stats
    int amount = 0;
    int amountSuccessFull = 0;
    int averageResponseTime = 0;
    
    // Loop until duration is reached
    while((DateTime.Now - startDate).TotalSeconds < durationInSeconds){
      // Run function
      DateTime startRequest = DateTime.Now;
      bool isSuccessFull = await test();

      // Calculate stats
      TimeSpan responseTime = DateTime.Now - startRequest;
      averageResponseTime = (averageResponseTime * amount + (int)responseTime.TotalMilliseconds) / (++amount);
      amountSuccessFull += isSuccessFull ? 1 : 0;

      // Wait for timeout
      await Task.Delay(timeoutInMilliSeconds);
    }

    // Return stats
    return new PerformanceResult {
      averageResponseTime = TimeSpan.FromMilliseconds(averageResponseTime),
      amountFailed = amount - amountSuccessFull,
      amountSuccessFull =  amountSuccessFull
    };
  }
}