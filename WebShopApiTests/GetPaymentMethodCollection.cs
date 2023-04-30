global using NUnit.Framework;
using System;
using ServiceReference;

namespace WebShopApiTests
{
  [TestFixture]
  public class GetPaymentMethodCollection
  {
    private readonly NopServiceClient _client;
    String username = Environment.GetEnvironmentVariable("API_USERNAME") ?? "jbtwaalf@gmail.com";
    String password = Environment.GetEnvironmentVariable("API_PASSWORD") ?? "Start1234%";

    public GetPaymentMethodCollection()
    {
      _client = new NopServiceClient();
    }

    [Test]
    public async Task TestValidCredentials_HappyFlow()
    {

      // Act
      var result = await _client.GetPaymentMethodCollectionAsync(username, password);

      // Assert
      Console.WriteLine("result " + result);
      Assert.IsNotEmpty(result);
    }

  }
}
