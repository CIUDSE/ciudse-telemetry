#include <Arduino.h>
#include <TinyGPSPlus.h>

#define gpsSerial Serial8   // GPS conectado al puerto serial 8 del Teensy

TinyGPSPlus gps;  // <<< FALTABA ESTA LÍNEA
String convertToDMS(double coord, bool isLatitude);

void setup() {
  Serial.begin(9600); 
  gpsSerial.begin(9600); 
  Serial.println("Esperando datos del GPS...");
}

void loop() {
  while (gpsSerial.available() > 0) {
    gps.encode(gpsSerial.read());

    if (gps.location.isUpdated()) {
      double lat = gps.location.lat();
      double lng = gps.location.lng();
      double alt = gps.altitude.meters();

      // Convertir a DMS
      String latDMS = convertToDMS(lat, true);
      String lngDMS = convertToDMS(lng, false);

      // Mostrar datos
      Serial.print(latDMS);
      Serial.print("    ");
      Serial.print(lngDMS);
      Serial.print("    ");
      Serial.print(alt, 1);
      Serial.println(" m");
    }
  }
}

String convertToDMS(double coord, bool isLatitude) {
  char hemisphere;
  if (isLatitude) {
    hemisphere = (coord >= 0) ? 'N' : 'S';
  } else {
    hemisphere = (coord >= 0) ? 'E' : 'W';
  }

  coord = abs(coord);
  int degrees = int(coord);
  double fractional = coord - degrees;
  int minutes = int(fractional * 60);
  double seconds = (fractional * 60 - minutes) * 60;

  char buffer[25];
  sprintf(buffer, "%d°%d'%d\" %c", degrees, minutes, int(seconds), hemisphere);
  return String(buffer);
}
