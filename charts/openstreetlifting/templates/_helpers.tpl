{{- define "openstreetlifting.labels" -}}
helm.sh/chart: {{ .Chart.Name }}-{{ .Chart.Version }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "openstreetlifting.frontend.selectorLabels" -}}
app.kubernetes.io/name: openstreetlifting-frontend
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "openstreetlifting.backend.selectorLabels" -}}
app.kubernetes.io/name: openstreetlifting-backend
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}
