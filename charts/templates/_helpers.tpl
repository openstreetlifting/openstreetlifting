{{- define "openstreetlifting.image" -}}
{{ .image.repository }}:{{ .image.tag | default .chart.AppVersion }}
{{- end }}

{{- define "openstreetlifting.labels" -}}
helm.sh/chart: {{ .Chart.Name }}-{{ .Chart.Version }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{- define "openstreetlifting.frontend.selectorLabels" -}}
app.kubernetes.io/name: openstreetlifting-frontend
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "openstreetlifting.backend.selectorLabels" -}}
app.kubernetes.io/name: openstreetlifting-backend
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "openstreetlifting.importer.selectorLabels" -}}
app.kubernetes.io/name: openstreetlifting-importer
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "openstreetlifting.importer.podSpec" -}}
{{- $root := .root -}}
restartPolicy: Never
containers:
  - name: importer
    image: "{{ include "openstreetlifting.image" (dict "image" $root.Values.importer.image "chart" $root.Chart) }}"
    imagePullPolicy: {{ $root.Values.importer.image.pullPolicy }}
    args:
      {{- toYaml .args | nindent 6 }}
    env:
      - name: LOG_FORMAT
        value: "json"
      - name: OTEL_SERVICE_NAME
        value: "osl-importer"
      - name: OTEL_EXPORTER_OTLP_ENDPOINT
        value: {{ $root.Values.telemetry.otlpEndpoint | quote }}
      - name: OTEL_EXPORTER_OTLP_PROTOCOL
        value: "http/protobuf"
      - name: OTEL_TRACES_SAMPLER
        value: {{ $root.Values.telemetry.traceSampler | quote }}
      - name: OTEL_TRACES_SAMPLER_ARG
        value: {{ $root.Values.telemetry.traceSampleRatio | quote }}
      - name: OTEL_RESOURCE_ATTRIBUTES
        value: {{ printf "deployment.environment.name=%s" (default "development" $root.Values.environment) | quote }}
      - name: DATABASE_URL
        valueFrom:
          secretKeyRef:
            name: {{ $root.Values.backend.database.secretName }}
            key: {{ $root.Values.backend.database.urlKey }}
      {{- if $root.Values.importer.privacy.secretName }}
      - name: OSL_PRIVACY_KEY
        valueFrom:
          secretKeyRef:
            name: {{ $root.Values.importer.privacy.secretName }}
            key: {{ $root.Values.importer.privacy.key }}
      {{- end }}
    resources:
      {{- toYaml $root.Values.importer.resources | nindent 6 }}
{{- end }}
