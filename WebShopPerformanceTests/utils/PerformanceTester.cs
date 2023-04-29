namespace WebShopPerformanceTests;

public static class PerformanceTester
{

  public partial class PerformanceResult
  {
    public TimeSpan averageResponseTime = TimeSpan.Zero;
    public int amountFailed = 0;

    public int amountSuccessFull = 0;
  }

  public partial class ConcurrentResult {
    public bool isSuccessFull = false;
    public int responseTime = 0;
  }

  /*
    Tests a service by sending requests with a timeout over a duration.
    Returns a PerformanceResult with the average response time, amount of failed requests and amount of successfull requests.
  */
  public static async Task<PerformanceResult> testLinear(int durationInSeconds, int timeoutInSeconds, Func<Task<bool>> test)
  {
    DateTime startDate = DateTime.Now;

    // Initialize stats
    int amount = 0;
    int amountSuccessFull = 0;
    int averageResponseTime = 0;

    // Loop until duration is reached
    while ((DateTime.Now - startDate).TotalSeconds < durationInSeconds)
    {
      // Run function
      DateTime startRequest = DateTime.Now;
      bool isSuccessFull = await test();

      // Calculate stats
      TimeSpan responseTime = DateTime.Now - startRequest;
      averageResponseTime = (averageResponseTime * amount + (int)responseTime.TotalMilliseconds) / (++amount);
      amountSuccessFull += isSuccessFull ? 1 : 0;

      // Wait for timeout
      await Task.Delay(timeoutInSeconds * 1000);
    }

    // Return stats
    return new PerformanceResult
    {
      averageResponseTime = TimeSpan.FromMilliseconds(averageResponseTime),
      amountFailed = amount - amountSuccessFull,
      amountSuccessFull = amountSuccessFull
    };
  }
  /*
    Tests a service by sending requests with a timeout over a duration with concurrency increasing every step.
    Returns a PerformanceResult with the average response time, amount of failed requests and amount of successfull requests.
  */
  public static async Task<PerformanceResult> testCascading(int durationInSeconds, int timeoutInSeconds, int maxConcurrent, Func<Task<bool>> test)
  {
    DateTime startDate = DateTime.Now;

    // Initialize stats
    int amount = 0;
    int step = 0;
    int amountSuccessFull = 0;
    int averageResponseTime = 0;

    // Calculate amount of steps
    int amountSteps = durationInSeconds / timeoutInSeconds;

    // Loop until duration is reached
    while ((DateTime.Now - startDate).TotalSeconds < durationInSeconds)
    {
      // Run function with concurrency
      int concurrency = calculateConcurrency(step++, maxConcurrent, amountSteps);
      Console.WriteLine("Concurrency: " + concurrency);
      List<Task<ConcurrentResult>> tasks = new List<Task<ConcurrentResult>>();

      for (int i = 0; i < concurrency; i++)
      {
        tasks.Add(runConcurrent(test));
      }

      ConcurrentResult[] results = await Task.WhenAll(tasks);

      // Calculate stats
      foreach (ConcurrentResult result in results)
      {
        averageResponseTime = (averageResponseTime * amount + result.responseTime) / (++amount);
        amountSuccessFull += result.isSuccessFull ? 1 : 0;
      }

      // Wait for timeout
      await Task.Delay(timeoutInSeconds * 1000);
    }

    // Return stats
    return new PerformanceResult
    {
      averageResponseTime = TimeSpan.FromMilliseconds(averageResponseTime),
      amountFailed = amount - amountSuccessFull,
      amountSuccessFull = amountSuccessFull
    };
  }

  /*
    Runs a function and returns a ConcurrentResult with the response time and if the request was successfull.
  */
  private static async Task<ConcurrentResult> runConcurrent(Func<Task<bool>> test)
  {
    DateTime startRequest = DateTime.Now;
    bool isSuccessFull = await test();

    return new ConcurrentResult
    {
      isSuccessFull = isSuccessFull,
      responseTime = (int)(DateTime.Now - startRequest).TotalMilliseconds
    };
  }

  /*
    Calculates the concurrency based on the step, maxconcurrency and amount of steps.
    First step is always 1 and last step is always maxconcurrency. (Latency is not taken into account)
  */
  private static int calculateConcurrency(int step, int maxConcurrent, int amountSteps)
  {
    return (int)Math.Round(((maxConcurrent - 1) / (double)(amountSteps - 1)) * step) + 1;
  }

}