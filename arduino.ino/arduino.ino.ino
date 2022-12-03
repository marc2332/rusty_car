int motorB1 = 4;
int motorB2 = 5;

void setup(){
  Serial.begin(9600);
  
  pinMode(motorB1, OUTPUT);
  pinMode(motorB2, OUTPUT);

  analogWrite(3, 255);
}

void loop(){
  if(Serial.available() > 0){
    char value = Serial.read();
    Serial.println(value);
    
    if(value == 'F') {
      digitalWrite(motorB1, HIGH);
      digitalWrite(motorB2, LOW);
    }
    else if(value == 'S') {
      digitalWrite(motorB1, LOW);
      digitalWrite(motorB2, LOW);
    }
  }                            
}    
