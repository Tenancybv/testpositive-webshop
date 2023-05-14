namespace WebShopApiTests;
using NUnit.Framework;
using System;
using ServiceReference;
using System.ServiceModel;

public class GetPaymentMethodCollection
{
  private readonly NopServiceClient _client;
  String username = Environment.GetEnvironmentVariable("API_USERNAME") ?? "";
  String password = Environment.GetEnvironmentVariable("API_PASSWORD") ?? "";

  private static readonly HashSet<string> ValidPaymentMethods = new HashSet<string>
    {
        "Payments.CashOnDelivery",
        "Payments.CheckMoneyOrder",
        "Payments.Manual",
        "Payments.PurchaseOrder"
    };

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
    Assert.IsNotEmpty(result);

    // Check if specific payment methods are present
    CollectionAssert.IsSubsetOf(ValidPaymentMethods, result.Select(p => p.Name).ToList());

    // Check if the list does not contain any unexpected or invalid payment methods
    foreach (var paymentMethod in result)
    {
      Assert.IsTrue(ValidPaymentMethods.Contains(paymentMethod.Name), $"Unexpected payment method found: {paymentMethod.Name}");
    }
  }

  [Test]
  public async Task TestInvalidUsername()
  {
    // Arrange
    string invalidUsername = "incorrect_username";

    // Act & Assert
    var exception = Assert.ThrowsAsync<FaultException<ExceptionDetail>>(async () => await _client.GetPaymentMethodCollectionAsync(invalidUsername, password));
    StringAssert.Contains("Not allowed", exception?.Message);

  }

  [Test]
  public async Task TestInvalidPassword()
  {
    // Arrange
    string invalidPassword = "incorrect_password";

    // Act & Assert
    var exception = Assert.ThrowsAsync<FaultException<ExceptionDetail>>(async () => await _client.GetPaymentMethodCollectionAsync(username, invalidPassword));
    StringAssert.Contains("Not allowed", exception?.Message);

  }

  [Test]
  public async Task TestNullCredentials_SOAPError()
  {
    // Act & Assert
    var exception = Assert.ThrowsAsync<FaultException<ExceptionDetail>>(async () => await _client.GetPaymentMethodCollectionAsync(null, null));
    StringAssert.Contains("Not allowed", exception?.Message);
  }

  // Further assertions...?



}

