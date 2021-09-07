#
# radio-thermostat-exporter
#
FROM python:slim
WORKDIR work
RUN useradd -ms /bin/bash thermostat
USER thermostat
COPY radio-thermostat-exporter.py .
RUN pip install tornado radiotherm
	
CMD [ "python", "/work/radio-thermostat-exporter.py" ]
