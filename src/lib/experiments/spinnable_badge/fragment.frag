uniform vec3 diffuse;
uniform vec3 lightPosition;  // NEU: Position des Lichts im Weltraum
uniform vec3 lightColor;
uniform vec3 ambientColor;
uniform float ambientIntensity;
uniform float lightRadius;   // NEU: Radius/Einflussbereich des Lichts für den Falloff (optional)

varying vec3 vNormal;
varying vec3 vWorldPosition; // Weltposition des Fragments

void main() {
    vec3 normal = normalize(vNormal);

    // --- 1. Ambient Light Komponente ---
    vec3 ambientLight = ambientColor * ambientIntensity;

    // --- 2. Diffuse Light Komponente (Point Light) ---
    // Richtung vom Fragment zum Lichtpunkt
    vec3 lightDirectionToFragment = normalize(lightPosition - vWorldPosition);

    // Distanz vom Fragment zum Lichtpunkt
    float distance = length(lightPosition - vWorldPosition);

    // Lambert'sches Cosinus-Gesetz (diffuse Komponente)
    // Wir nutzen hier `lightDirectionToFragment`, da sie VOM Licht zum Fragment zeigt.
    // Wenn `normal` nach außen zeigt, wollen wir den Winkel zur entgegen gesetzten Richtung der Lichtquelle.
    // Oder, wir nutzen den Vektor VOM Fragment ZUM Licht und dotten ihn mit der Normalen.
    // Hier ist es `dot(normal, lightDirectionToFragment)`, weil `lightDirectionToFragment`
    // die Richtung ist, in die das Licht STRAHLT (aus Sicht des Fragments).
    float NdotL = max(dot(normal, lightDirectionToFragment), 0.0);

    // Licht-Abschwächung (Falloff/Attenuation)
    // Ein gängiges quadratisches Falloff-Modell: 1.0 / (a + b*d + c*d*d)
    // Für einen einfacheren, aber effektiven Falloff, können wir auch einen Radius nutzen:
    // Der LightRadius bestimmt, wie weit das Licht wirkt und wie schnell es abnimmt.
    float attenuation = 1.0 / (1.0 + (distance * distance) / (lightRadius * lightRadius));
    // Oder ein einfaches inverses Quadrat:
    // float attenuation = 1.0 / (distance * distance + 0.0001); // 0.0001 um Division durch Null zu vermeiden

    // Kombiniere NdotL mit der Abschwächung
    float directLightIntensity = NdotL * attenuation;

    vec3 diffuseLight = diffuse * lightColor * directLightIntensity;

    // --- 3. Endgültige Farbe ---
    vec3 finalColor = ambientLight + diffuseLight;
    finalColor = clamp(finalColor, 0.0, 1.0);

    gl_FragColor = vec4(finalColor, 1.0);
}