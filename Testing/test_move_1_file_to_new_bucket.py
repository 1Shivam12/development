import pytest
import move_1_file_to_new_bucket
import boto3
from moto import mock_s3
class TestMoveFile:

    @mock_s3
    def test_move_1_file_to_new_bucket(self, monkeypatch):

        # Create a s3 resource
        s3_resource = boto3.resource("s3")

        # Create dummy buckets to use in test
        s3_resource.create_bucket(Bucket='source_bucket')
        s3_resource.create_bucket(Bucket='target_bucket')

        # Upload a file to the source bucket
        file_data = b'This is my source file data'
        file_obj = s3_resource.Object('source_bucket', 'source_file')
        file_obj.put(Body=file_data)

        # Use monkeypatch to set the SSEKMS key to a dummy value
        monkeypatch.setenv('SSEKMSKeyId', 'ssekmskey')

        # Run the function being tested
        move_1_file_to_new_bucket.move_1_file_to_new_bucket(
            source_s3_bucket='source_bucket',
            source_s3_file='source_file',
            destination_s3_bucket='target_bucket',
            destination_s3_file='target_file'
        )

        # List contents of target bucket (to check if file was uploaded)
        target_bucket = s3_resource.Bucket('target_bucket')
        files_in_target = []
        for my_bucket_object in target_bucket.objects.all():
            files_in_target.append(my_bucket_object.key)
            

        assert 'source_file' in files_in_target


        
