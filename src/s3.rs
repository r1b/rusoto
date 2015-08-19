//! S3 bindings for Rust
#![allow(unused_variables, unused_mut)]
use credentials::*;
use xml::*;
use signature::*;
use params::*;
use error::*;
use xmlutil::*;
use std::str::FromStr;
// use hyper::header::Headers;
use hyper::client::Response;
use std::io::Read;

// include the code generated from the SQS botocore templates
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/codegen/s3.rs"));

pub struct S3Helper<'a> {
	client: S3Client<'a>
}

impl<'a> S3Helper<'a> {
	pub fn new(credentials:&'a AWSCredentials, region:&'a str) -> S3Helper<'a> {
		S3Helper { client: S3Client::new(credentials, region) }
	}

	pub fn list_buckets(&self) -> Result<ListBucketsOutput, AWSError> {
		self.client.list_buckets()
	}

	pub fn create_bucket(&self, bucket_name: &str) -> Result<CreateBucketOutput, AWSError> {
		let mut request = CreateBucketRequest::default();
		request.bucket = bucket_name.to_string();
		// println!("Creating bucket");
		let result = self.client.create_bucket(&request);
		// println!("Result is {:?}", result);
		result
	}

	pub fn delete_bucket(&self, bucket_name: &str) -> Result<(), AWSError> {
		let mut request = DeleteBucketRequest::default();
		request.bucket = bucket_name.to_string();
		println!("Deleting bucket");
		let result = self.client.delete_bucket(&request);
		println!("Result is {:?}", result);
		result
	}

	pub fn get_object(&self, bucket_name: &str, object_name: &str) ->  Result<GetObjectOutput, AWSError> {
		let mut request = GetObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		let result = self.client.get_object(&request);
		// println!("Result is {:?}", result);
		result
	}

	pub fn put_object(&self, bucket_name: &str, object_name: &str, object_as_bytes: &Vec<u8>) ->  Result<PutObjectOutput, AWSError> {
		let mut request = PutObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		request.body = Some(object_as_bytes.clone()); // this needs to be refactored to pass a reference
		let result = self.client.put_object(&request);
		// println!("Result is {:?}", result);
		result
	}

	pub fn delete_object(&self, bucket_name: &str, object_name: &str) ->  Result<DeleteObjectOutput, AWSError> {
		let mut request = DeleteObjectRequest::default();
		request.key = object_name.to_string();
		request.bucket = bucket_name.to_string();
		let result = self.client.delete_object(&request);
		println!("Result is {:?}", result);
		result
	}
}

#[cfg(test)]
mod tests {
	use xml::reader::*;
	use std::io::BufReader;
	use std::fs::File;
	use super::ListBucketsOutputParser;
	use xmlutil::*;

	#[test]
	fn list_buckets_happy_path() {
		let file = File::open("tests/sample-data/s3_get_buckets.xml").unwrap();
	    let file = BufReader::new(file);
	    let mut my_parser  = EventReader::new(file);
	    let my_stack = my_parser.events().peekable();
	    let mut reader = XmlResponseFromFile::new(my_stack);
		reader.next(); // xml start node
		let result = ListBucketsOutputParser::parse_xml("ListAllMyBucketsResult", &mut reader);

		match result {
			Err(_) => panic!("Couldn't parse list_buckets"),
			Ok(_) => return,
		}
	}
}