namespace WebShopApiTests;
using NUnit.Framework;
using System;
using ServiceReference;
using System.ServiceModel;
using System.Text;
using System.Net;

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
  public async Task TestInvalidSoapRequest()
  {
    string soapRequest = @"<soapenv:Envelope xmlns:soapenv=""http://schemas.xmlsoap.org/soap/envelope/"" xmlns:web=""http://tempuri.org/"">
   <soapenv:Header/>
   <soapenv:Body>
      <web:GetPaymentMethodCollection>
         <web:usernameOrEmail>test@test.com</web:usernameOrEmil>
         <web:userPassword>password</web:userPassword>
      </web:GetPaymentMethodCollection>
   </soapenv:Body>
</soapenv:Envelope>";

    HttpClient client = new HttpClient();
    HttpRequestMessage request = new HttpRequestMessage(HttpMethod.Post, "http://demowebshop.tricentis.com/Plugins/Misc.WebServicesCustomer/Remote/NopService.svc");
    request.Content = new StringContent(soapRequest, Encoding.UTF8, "text/xml");
    request.Headers.Add("SOAPAction", "http://tempuri.org/INopService/GetPaymentMethodCollection");

    // Act
    HttpResponseMessage response = await client.SendAsync(request);
    string responseContent = await response.Content.ReadAsStringAsync();

    // Assert
    StringAssert.Contains("an error deserializing the object", responseContent);
    Assert.AreEqual(HttpStatusCode.InternalServerError, response.StatusCode);
  }



}

