import os

import click
import cv2
from imutils.perspective import four_point_transform
import numpy as np
from PIL import Image
from PIL import ImageDraw
from scipy.ndimage import center_of_mass
from skimage.morphology import remove_small_objects
import tensorflow as tf
from tensorflow import keras
from keras import layers
import itertools
import matplotlib.pyplot as plt

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'
tf.get_logger().setLevel('ERROR')


def print_img(im, figsize=(5,5)):
    cv2.namedWindow("output", cv2.WINDOW_NORMAL)
    cv2.imshow('output', im)
    cv2.waitKey(0)
    cv2.destroyAllWindows()


def crop_image(img, scale=1.0):
    center_x, center_y = img.shape[1] / 2, img.shape[0] / 2
    width_scaled, height_scaled = img.shape[1] * scale, img.shape[0] * scale
    left_x, right_x = center_x - width_scaled / 2, center_x + width_scaled / 2
    top_y, bottom_y = center_y - height_scaled / 2, center_y + height_scaled / 2
    img_cropped = img[int(top_y) : int(bottom_y), int(left_x) : int(right_x)]
    return img_cropped


def find_puzzle(image, *, zoom=0.95, gaussian_kernel=21, dilate_kernel=1, debug=False):
    # convert the image to grayscale and blur it slightly
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    # TODO parameterize kernel sizes - we need ~40-ish blur, 5 dilate for photos
    # and ~20-ish blur, 1 dilate for screenshots
    blurred = cv2.GaussianBlur(gray, (gaussian_kernel, gaussian_kernel), 4)
    # apply adaptive thresholding and then invert the threshold map
    thresh = cv2.adaptiveThreshold(blurred, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 2)
    thresh = cv2.bitwise_not(thresh)
    # check to see if we are visualizing each step of the image
    # processing pipeline (in this case, thresholding)
    if debug:
        print_img(thresh)
    dilated = cv2.dilate(thresh, kernel=np.ones((dilate_kernel, dilate_kernel), np.uint8), iterations=4)
    if debug:
        print_img(dilated)
    cnts = cv2.findContours(dilated.copy(), cv2.RETR_CCOMP, cv2.CHAIN_APPROX_SIMPLE)
    # cnts = imutils.grab_contours(cnts)
    cnts_sorted = sorted(zip(cnts[0], cnts[1][0]), key=lambda x: cv2.contourArea(x[0]), reverse=True)
    # initialize a contour that corresponds to the puzzle outline
    puzzleCnt = []
    puzzleHs = []
    # loop over the contours
    for c, hierarchy in cnts_sorted:
        # approximate the contour
        peri = cv2.arcLength(c, True)
        approx = cv2.approxPolyDP(c, 0.02 * peri, True)
        # if our approximated contour has four points, then we can
        # assume we have found the outline of the puzzle
        if len(approx) == 4 and hierarchy[3] >= 0:
            puzzleCnt.append(approx)
            puzzleHs.append(hierarchy)
            # break
        # if the puzzle contour is empty then our script could not find
    if puzzleCnt is None:
        raise Exception(("Could not find Slitherlinker puzzle outline. " "Try debugging your thresholding and contour steps."))
    # check to see if we are visualizing the outline of the detected
    # Sudoku puzzle
    if debug:
        # draw the contour of the puzzle on the image and then display
        # it to our screen for visualization/debugging purposes
        output = image.copy()
        cv2.drawContours(output, puzzleCnt, -1, (0, 255, 0), 4)
        print_img(output)
    puzzle = four_point_transform(image, puzzleCnt[0].reshape(4, 2))
    warped = four_point_transform(gray, puzzleCnt[0].reshape(4, 2))
    if debug:
        print_img(warped)

    puzzle = crop_image(puzzle, zoom)
    warped = crop_image(warped, zoom)

    if debug:
        print_img(warped)
    return puzzle, warped


def translate_to_com(im, com):
    x_trans = int(im.shape[0] // 2 - com[0])
    y_trans = int(im.shape[1] // 2 - com[1])

    # Pad and remove pixels from image to perform translation

    if x_trans > 0:
        im2 = np.pad(im, ((x_trans, 0), (0, 0)), mode='constant')
        im2 = im2[: im.shape[0] - x_trans, :]
    else:
        im2 = np.pad(im, ((0, -x_trans), (0, 0)), mode='constant')
        im2 = im2[-x_trans:, :]

    if y_trans > 0:
        im3 = np.pad(im2, ((0, 0), (y_trans, 0)), mode='constant')
        im3 = im3[:, : im.shape[0] - y_trans]

    else:
        im3 = np.pad(im2, ((0, 0), (0, -y_trans)), mode='constant')
        im3 = im3[:, -y_trans:]

    return im3


def prepare_digit(cell, cell_w, cell_h, debug, small_treshold, gray_treshold):
    sub_img = cell
    if debug:
        print('gray image:')
        print_img(sub_img)
    if np.sum(sub_img) > 0:
        sub_img = cv2.GaussianBlur(sub_img, (11, 11), 2)

        if debug:
            print('blurred image:')
            print_img(sub_img)
    sub_img = sub_img > gray_treshold
    if debug:
        print('bool image:')
        print_img(sub_img.astype(np.uint8) * 255)
    sub_img = remove_small_objects(~sub_img, min_size=(cell_w * cell_h * small_treshold)) * 255
    if np.sum(sub_img) > 0:
        com = center_of_mass(sub_img)
        sub_img = translate_to_com(sub_img, com)
    if debug:
        print('center image:')
        print_img(sub_img.astype(np.uint8))
    if np.sum(sub_img) == 0:
        sub_img = np.zeros((28, 28), dtype=np.uint8)
    else:
        sub_img = cv2.resize(sub_img.astype(np.uint8), dsize=(28, 28))
    if debug:
        print('resized image:')
        print_img(sub_img)
    sub_img = sub_img.reshape((28, 28, 1))
    return sub_img


def get_not_empty_cells(image, x_size, y_size, debug, small_treshold, gray_treshold):
    h, w = image.shape
    if debug:
        print(f'Image shape: {image.shape}')
    cell_w, cell_h = int(w / x_size), int(h / y_size)
    indices = []
    for i in range(y_size):
        for j in range(x_size):
            sub_img = image[cell_h * i : cell_h * (i + 1), cell_w * j : cell_w * (j + 1)]
            cleaned = prepare_digit(sub_img, cell_w, cell_h, debug, small_treshold, gray_treshold)
            if np.sum(cleaned) > 0:
                indices.append((i, j))

    return indices


def get_model():
    model = keras.Sequential(
        [
            keras.Input(shape=(28, 28, 1)),
            layers.Conv2D(32, kernel_size=(3, 3), activation="relu"),
            layers.MaxPooling2D(pool_size=(2, 2)),
            layers.Conv2D(64, kernel_size=(3, 3), activation="relu"),
            layers.MaxPooling2D(pool_size=(2, 2)),
            layers.Flatten(),
            layers.Dropout(0.5),
            layers.Dense(4, activation="softmax"),
        ]
    )
    model.compile(loss="categorical_crossentropy", optimizer="adam", metrics=["accuracy"])
    model.load_weights('model.hd5')
    return model


def recognize_digits(warped, non_empty, x_size, y_size, debug, small_treshold, gray_treshold):
    fig, axs = plt.subplots(6,6, figsize=(10, 10))
    model = get_model()
    h, w = warped.shape
    cell_w, cell_h = int(w / x_size), int(h / y_size)
    res = []
    for i, j in non_empty:
        sub_img = warped[cell_h * i : cell_h * (i + 1), cell_w * j : cell_w * (j + 1)]
        sub_img = prepare_digit(sub_img, cell_w, cell_h, debug, small_treshold, gray_treshold)
        pred_vector = model.predict(np.array([sub_img > 0]), verbose=debug)[0]
        prediction = np.argmax(pred_vector)
        if debug:
            print(f'Prediction {i,j}: {prediction} ({pred_vector[prediction]})')
            axs[i][j].imshow(sub_img / 255.)
        res.append(prediction)

    if debug:
        plt.draw()
        plt.show()
    return res


def draw_grid_puzzle(img, x_size, y_size):
    h, w = img.shape
    cell_w, cell_h = int(w / x_size), int(h / y_size)
    im = Image.fromarray(img.copy())
    draw = ImageDraw.Draw(im)

    for x in range(0, w, cell_w):

        line = ((x, 0), (x, h))
        draw.line(line, fill=128)

    for x in range(0, h, cell_h):
        line = ((0, x), (w, x))
        draw.line(line, fill=128)
    del draw
    im.show()

def serialize_slitherlinker_puzzle(matrix):
    xs, ys = matrix.shape
    # flatten matrix
    flat = matrix.reshape(xs * ys)
    prefix = f'{xs}x{ys}:'
    # groupby 0s that are next to each other
    groups = []
    for k, g in itertools.groupby(flat):
        groups.append(list(g))

    # serialize
    res = prefix
    for g in groups:
        head = g[0]
        if head == -1:
            count = len(g)
            res += chr(ord('a') - 1 + count)
        else:
            res += str(head) * len(g)
    return res



@click.command()
@click.option('--img', required=True)
@click.option('--debug', is_flag=True)
@click.option('--x-size', type=int, default=6)
@click.option('--y-size', type=int, default=6)
@click.option('--zoom', type=float, default=0.95)
@click.option('--small-treshold', type=float, default=0.04)
@click.option('--gray-treshold', type=int, default=160)
def detect(img, x_size, y_size, debug, zoom, small_treshold, gray_treshold):
    im = cv2.imread(img)
    if debug:
        print('debug. initial image:')
        print_img(im)

    puzzle, warped = find_puzzle(im, zoom=zoom, debug=debug)
    if debug:
        draw_grid_puzzle(warped, x_size, y_size)
    non_empty = get_not_empty_cells(warped, x_size, y_size, False, small_treshold, gray_treshold)

    detected_digits = recognize_digits(warped, non_empty, x_size, y_size, debug, small_treshold, gray_treshold)
    if debug:
        print(f'Not empty cells at: {non_empty}')
        print(f'Digits are: {detected_digits}')

    result = np.ones((y_size, x_size), dtype=np.int8) * -1

    for (i, j), d in zip(non_empty, detected_digits):
        result[i][j] = d

    if debug:
        print(result.tolist())

    print("Visual rep of the puzzle:")
    for i in range(y_size):
        for j in range(x_size):
            if result[i][j] < 0:
                print('.', end=' ')
            else:
                print(f'{result[i][j]}', end=' ')
        print()

    print("Puzzle Code: ", serialize_slitherlinker_puzzle(result))


if __name__ == "__main__":
    detect()
