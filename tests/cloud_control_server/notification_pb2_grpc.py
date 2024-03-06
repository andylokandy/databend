# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import notification_pb2 as notification__pb2


class NotificationServiceStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateNotification = channel.unary_unary(
            "/notificationproto.NotificationService/CreateNotification",
            request_serializer=notification__pb2.CreateNotificationRequest.SerializeToString,
            response_deserializer=notification__pb2.CreateNotificationResponse.FromString,
        )
        self.DropNotification = channel.unary_unary(
            "/notificationproto.NotificationService/DropNotification",
            request_serializer=notification__pb2.DropNotificationRequest.SerializeToString,
            response_deserializer=notification__pb2.DropNotificationResponse.FromString,
        )
        self.ListNotification = channel.unary_unary(
            "/notificationproto.NotificationService/ListNotification",
            request_serializer=notification__pb2.ListNotificationRequest.SerializeToString,
            response_deserializer=notification__pb2.ListNotificationResponse.FromString,
        )
        self.GetNotification = channel.unary_unary(
            "/notificationproto.NotificationService/GetNotification",
            request_serializer=notification__pb2.GetNotificationRequest.SerializeToString,
            response_deserializer=notification__pb2.GetNotificationResponse.FromString,
        )
        self.AlterNotification = channel.unary_unary(
            "/notificationproto.NotificationService/AlterNotification",
            request_serializer=notification__pb2.AlterNotificationRequest.SerializeToString,
            response_deserializer=notification__pb2.AlterNotificationResponse.FromString,
        )
        self.ListNotificationHistory = channel.unary_unary(
            "/notificationproto.NotificationService/ListNotificationHistory",
            request_serializer=notification__pb2.ListNotificationHistoryRequest.SerializeToString,
            response_deserializer=notification__pb2.ListNotificationHistoryResponse.FromString,
        )


class NotificationServiceServicer(object):
    """Missing associated documentation comment in .proto file."""

    def CreateNotification(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def DropNotification(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def ListNotification(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def GetNotification(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def AlterNotification(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def ListNotificationHistory(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")


def add_NotificationServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {
        "CreateNotification": grpc.unary_unary_rpc_method_handler(
            servicer.CreateNotification,
            request_deserializer=notification__pb2.CreateNotificationRequest.FromString,
            response_serializer=notification__pb2.CreateNotificationResponse.SerializeToString,
        ),
        "DropNotification": grpc.unary_unary_rpc_method_handler(
            servicer.DropNotification,
            request_deserializer=notification__pb2.DropNotificationRequest.FromString,
            response_serializer=notification__pb2.DropNotificationResponse.SerializeToString,
        ),
        "ListNotification": grpc.unary_unary_rpc_method_handler(
            servicer.ListNotification,
            request_deserializer=notification__pb2.ListNotificationRequest.FromString,
            response_serializer=notification__pb2.ListNotificationResponse.SerializeToString,
        ),
        "GetNotification": grpc.unary_unary_rpc_method_handler(
            servicer.GetNotification,
            request_deserializer=notification__pb2.GetNotificationRequest.FromString,
            response_serializer=notification__pb2.GetNotificationResponse.SerializeToString,
        ),
        "AlterNotification": grpc.unary_unary_rpc_method_handler(
            servicer.AlterNotification,
            request_deserializer=notification__pb2.AlterNotificationRequest.FromString,
            response_serializer=notification__pb2.AlterNotificationResponse.SerializeToString,
        ),
        "ListNotificationHistory": grpc.unary_unary_rpc_method_handler(
            servicer.ListNotificationHistory,
            request_deserializer=notification__pb2.ListNotificationHistoryRequest.FromString,
            response_serializer=notification__pb2.ListNotificationHistoryResponse.SerializeToString,
        ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
        "notificationproto.NotificationService", rpc_method_handlers
    )
    server.add_generic_rpc_handlers((generic_handler,))


# This class is part of an EXPERIMENTAL API.
class NotificationService(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def CreateNotification(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/notificationproto.NotificationService/CreateNotification",
            notification__pb2.CreateNotificationRequest.SerializeToString,
            notification__pb2.CreateNotificationResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )

    @staticmethod
    def DropNotification(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/notificationproto.NotificationService/DropNotification",
            notification__pb2.DropNotificationRequest.SerializeToString,
            notification__pb2.DropNotificationResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )

    @staticmethod
    def ListNotification(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/notificationproto.NotificationService/ListNotification",
            notification__pb2.ListNotificationRequest.SerializeToString,
            notification__pb2.ListNotificationResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )

    @staticmethod
    def GetNotification(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/notificationproto.NotificationService/GetNotification",
            notification__pb2.GetNotificationRequest.SerializeToString,
            notification__pb2.GetNotificationResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )

    @staticmethod
    def AlterNotification(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/notificationproto.NotificationService/AlterNotification",
            notification__pb2.AlterNotificationRequest.SerializeToString,
            notification__pb2.AlterNotificationResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )

    @staticmethod
    def ListNotificationHistory(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/notificationproto.NotificationService/ListNotificationHistory",
            notification__pb2.ListNotificationHistoryRequest.SerializeToString,
            notification__pb2.ListNotificationHistoryResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
        )
