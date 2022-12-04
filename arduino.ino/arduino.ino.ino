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
    int value = Serial.read();
    Serial.println(value);
    int minimumVelocity = 60;
    int velocity = minimumVelocity + ((255 - minimumVelocity) / 100 * value);

    analogWrite(3, velocity);
    
    if(velocity > 60) {
      digitalWrite(motorB1, HIGH);
      digitalWrite(motorB2, LOW);
    }
    else {
      digitalWrite(motorB1, LOW);
      digitalWrite(motorB2, LOW);
    }
  }                            
}    
