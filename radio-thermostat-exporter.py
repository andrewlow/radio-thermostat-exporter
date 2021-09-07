#!/usr/local/bin/python
#
# Currently exposes current tempurature and thermostat state
# Tempreature in in F
# State: 0:off, 1:heat, 2:cool
#
import tornado.ioloop
import tornado.web
import os
import radiotherm

url = os.environ['TSTAT']
tstat = radiotherm.get_thermostat(url)

class Exporter(tornado.web.RequestHandler):
    def get(self):
        metrics = []
        metrics.append( { 'type': 'temperature', 'value': tstat.temp['raw'] })
        metrics.append( { 'type': 'state', 'value': tstat.tstate['raw'] })

        for entry in metrics:
            self.write('radio_thermostat_%s %f\n'%(entry['type'], entry['value']))

def make_app():
    return tornado.web.Application([
        (r"/metrics", Exporter),
    ])

if __name__ == "__main__":
    app = make_app()
    app.listen(9864)
    tornado.ioloop.IOLoop.current().start()
