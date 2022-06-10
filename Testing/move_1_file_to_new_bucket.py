import boto3
import os
#@task(log_stdout=True)
def move_1_file_to_new_bucket(source_s3_bucket: str, source_s3_file, destination_s3_bucket: str,
                             destination_folder: str = '', destination_s3_file=None):
    """
    Move 1 file from one s3 bucket to another
    :param: source_s3_bucket: source bucket name
    :param: source_s3_file: source file path
    :param: destination_s3_bucket: destination bucket name
    :param: destination_folder
    :param: destination_s3_file: optional new file name
    :return: list of destination file paths
    """
    s3_wranglers_dev = boto3.resource('s3')
    if not isinstance(source_s3_file, str):
        raise Exception('multiple source file paths detected as list, this function only processes single files')
    if destination_s3_file is not None and not isinstance(destination_s3_file, str):
        raise Exception('multiple destination file paths detected as list, this function only processes single files')
    if destination_s3_file is not None:
        destination_key = destination_folder + source_s3_file
    try:
        copy_source = {
            'Bucket': source_s3_bucket,
            'Key': source_s3_file
        }
        encrypt_files = {
            'ServerSideEncryption': 'aws:kms',
            'SSEKMSKeyId': os.getenv('SSEKMSKeyId')
        }
        s3_wranglers_dev.meta.client.copy(copy_source, destination_s3_bucket, destination_key,
                                            ExtraArgs=encrypt_files)
    except Exception:
        print(f'Error moving {source_s3_bucket}/{source_s3_file} to {destination_s3_bucket}/{destination_key}')
        raise
    print(f'{source_s3_bucket}/{source_s3_file} copied to location: '
            f'{destination_s3_bucket}/{destination_key}')
    return destination_key