import grpc
from concurrent import futures
import image_stream_pb2
import image_stream_pb2_grpc
import os

class ImageStreamServicer(image_stream_pb2_grpc.ImageStreamServicer):
    def StreamImages(self, request, context):
        image_folder = "../images"
        for filename in os.listdir(image_folder):
            if filename.endswith(".jpg") or filename.endswith(".png"):
                with open(os.path.join(image_folder, filename), "rb") as image_file:
                    image_data = image_file.read()
                    yield image_stream_pb2.ImageData(name=filename, data=image_data)

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    image_stream_pb2_grpc.add_ImageStreamServicer_to_server(ImageStreamServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()

if __name__ == '__main__':
    serve()
