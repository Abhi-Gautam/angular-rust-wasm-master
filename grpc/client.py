import grpc
from flask import Flask, Response, jsonify, send_file
import image_stream_pb2
import image_stream_pb2_grpc
import os
import io
import logging

app = Flask(__name__)

logging.basicConfig(level=logging.DEBUG)

# Initialize gRPC client
channel = grpc.insecure_channel('localhost:50051')
stub = image_stream_pb2_grpc.ImageStreamStub(channel)

# Cache to store image data
image_cache = {}

@app.route('/images/<filename>')
def get_image(filename):
    if filename in image_cache:
        image_data = image_cache[filename]
        return Response(image_data, mimetype='image/jpeg')
    else:
        return jsonify({"error": "Image not found"}), 404

@app.route('/images')
def list_images():
    response = stub.StreamImages(image_stream_pb2.Empty())
    image_list = []
    for image in response:
        image_cache[image.name] = image.data
        image_list.append(image.name)
    return jsonify({"images": image_list})

@app.errorhandler(500)
def internal_error(error):
    logging.error(f"Server Error: {error}")
    return jsonify({"error": "Internal Server Error"}), 500

@app.errorhandler(Exception)
def unhandled_exception(error):
    logging.error(f"Unhandled Exception: {error}")
    return jsonify({"error": "Unhandled Exception"}), 500

if __name__ == '__main__':
    app.run(port=5000, debug=True)
