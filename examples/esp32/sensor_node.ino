/**
 * # OpenConstruct — ESP32 Sensor Node
 *
 * Registers an ESP32 as a "room" in the OpenConstruct fleet, reads
 * temperature and humidity from a DHT22 sensor, and publishes readings
 * to the coordinator over WiFi.
 *
 * ## Hardware
 *   - ESP32 dev board
 *   - DHT22 temperature/humidity sensor on GPIO 4
 *
 * ## Setup
 *   1. Install Arduino IDE + ESP32 board support
 *   2. Install libraries: WiFi, WebSockets, DHT sensor library
 *   3. Update WIFI_SSID, WIFI_PASS, COORDINATOR_URL below
 *   4. Flash to ESP32
 */

#include <WiFi.h>
#include <WebSocketsClient.h>
#include <DHT.h>

/* ── Configuration ────────────────────────────────────────────────── */
#define WIFI_SSID        "your-ssid"
#define WIFI_PASS        "your-password"
#define COORDINATOR_HOST "192.168.1.100"
#define COORDINATOR_PORT 9142
#define COORDINATOR_PATH "/ws"

#define DHT_PIN    4
#define DHT_TYPE   DHT22
#define ROOM_NAME  "esp32-sensor-room"
#define AGENT_NAME "esp32-sensor-node"

#define HEARTBEAT_MS  30000
#define READ_INTERVAL 5000

/* ── Globals ──────────────────────────────────────────────────────── */
static WebSocketsClient ws;
static DHT dht(DHT_PIN, DHT_TYPE);
static unsigned long last_heartbeat = 0;
static unsigned long last_read      = 0;

/* ── Send a JSON message to the coordinator ───────────────────────── */
static void send_json(const char *json) {
    Serial.printf("[WS→] %s\n", json);
    ws.sendTXT(json);
}

/* ── WebSocket event handler ──────────────────────────────────────── */
static void on_ws_event(WStype_t type, uint8_t *payload, size_t length) {
    (void)length;

    switch (type) {
        case WStype_DISCONNECTED:
            Serial.println("[WS] disconnected — will retry");
            break;

        case WStype_CONNECTED:
            Serial.printf("[WS] connected to %s\n", payload);
            /* Onboard immediately after connecting */
            {
                char msg[256];
                snprintf(msg, sizeof(msg),
                    "{\"type\":\"onboard\","
                    "\"name\":\"%s\","
                    "\"capabilities\":[\"sense\"],"
                    "\"room\":\"%s\","
                    "\"metadata\":{\"platform\":\"esp32\"}}",
                    AGENT_NAME, ROOM_NAME);
                send_json(msg);
            }
            break;

        case WStype_TEXT:
            Serial.printf("[WS←] %s\n", payload);
            /* In a full implementation, parse responses and handle
               heartbeat acknowledgements, rejections, etc. */
            break;

        default:
            break;
    }
}

/* ── Publish a sensor reading ─────────────────────────────────────── */
static void publish_reading(float temperature, float humidity) {
    char msg[256];
    snprintf(msg, sizeof(msg),
        "{\"type\":\"sense\","
        "\"sensor\":\"dht22\","
        "\"temperature_c\":%.2f,"
        "\"humidity_pct\":%.2f,"
        "\"confidence\":0.95}",
        temperature, humidity);
    send_json(msg);
}

/* ── Send heartbeat ───────────────────────────────────────────────── */
static void send_heartbeat(void) {
    send_json("{\"type\":\"heartbeat\"}");
}

/* ── Setup ────────────────────────────────────────────────────────── */
void setup() {
    Serial.begin(115200);
    Serial.println("\n=== OpenConstruct ESP32 Sensor Node ===");

    dht.begin();

    /* Connect to WiFi */
    WiFi.begin(WIFI_SSID, WIFI_PASS);
    Serial.print("[WiFi] connecting");
    int retries = 0;
    while (WiFi.status() != WL_CONNECTED && retries < 40) {
        delay(500);
        Serial.print(".");
        retries++;
    }

    if (WiFi.status() != WL_CONNECTED) {
        Serial.println("\n[WiFi] FAILED — restarting in 10s");
        delay(10000);
        ESP.restart();
    }

    Serial.printf("\n[WiFi] connected — IP: %s\n",
                  WiFi.localIP().toString().c_str());

    /* Set up WebSocket connection to coordinator */
    ws.begin(COORDINATOR_HOST, COORDINATOR_PORT, COORDINATOR_PATH);
    ws.onEvent(on_ws_event);
    ws.setReconnectInterval(5000);
}

/* ── Loop ─────────────────────────────────────────────────────────── */
void loop() {
    ws.loop();

    unsigned long now = millis();

    /* Heartbeat */
    if (now - last_heartbeat >= HEARTBEAT_MS) {
        send_heartbeat();
        last_heartbeat = now;
    }

    /* Sensor reading */
    if (now - last_read >= READ_INTERVAL) {
        float temp = dht.readTemperature();
        float hum  = dht.readHumidity();

        if (isnan(temp) || isnan(hum)) {
            Serial.println("[DHT] read failed — skipping");
        } else {
            Serial.printf("[DHT] %.1f°C / %.1f%%\n", temp, hum);
            publish_reading(temp, hum);
        }

        last_read = now;
    }
}
