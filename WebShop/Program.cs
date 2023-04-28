public static class Program {
  static void Main(string[] args) {
    //Run a command
    String command = args[0];

    switch(command) {
      case "calculateshipping":
        CalculateShippingCommand(args);
        break;
      default:
        Console.WriteLine("Command not known");
        break;
    }
  }

  static void CalculateShippingCommand(string[] args){
    String typeOfShippingCosts = args[1];
    Double totalPrice = Convert.ToDouble(args[2]);

    CostsCalculator calculator = new CostsCalculator();
    Double costs = calculator.ShippingCosts(true, typeOfShippingCosts, totalPrice);

    Console.WriteLine($"Costs: {costs}");
  }
}