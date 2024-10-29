const tokens=expression.trim().split(/\s+/).reverse();

  const eval =()=>{
    const token=tokens.pop();

    if(isNaN(token)){
      return parseFloat(token);
    }
    const operand1= eval();
    const operand2= eval();

    switch(token){
      case '+':
        return operand1+operand2;
      case '-':
        return operand1-operand2;
      case '*':
        return operand1*operand2;
      case '/':
        if(operand2===0){
          console.error("Division by zero");
          return null;
        }
        return operand1/operand2;

      default:
         console.error("Invalid operator");
         return null;

    }
  }

  try {
    const result=eval();
    if(tokens.length!==0){
      console.error("Invalid expression");
      return null;
    }
    return result;
  } catch (error) {
    console.error("Error during calculation:");
    return null;
  }