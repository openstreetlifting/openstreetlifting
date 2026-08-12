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

{{- define "openstreetlifting.importer.selectorLabels" -}}
app.kubernetes.io/name: openstreetlifting-importer
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "openstreetlifting.importer.podSpec" -}}
restartPolicy: Never
containers:
  - name: importer
    image: "{{ .Values.importer.image.repository }}:{{ .Values.importer.image.tag }}"
    imagePullPolicy: {{ .Values.importer.image.pullPolicy }}
    args:
      {{- toYaml .Values.importer.args | nindent 6 }}
    env:
      - name: DATABASE_URL
        valueFrom:
          secretKeyRef:
            name: {{ .Values.backend.database.secretName }}
            key: {{ .Values.backend.database.urlKey }}
    resources:
      {{- toYaml .Values.importer.resources | nindent 6 }}
{{- end }}
